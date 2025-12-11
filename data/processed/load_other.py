from neo4j import GraphDatabase
import csv

URI = "bolt://127.0.0.1:7687"
USER = "neo4j"
PASSWORD = "1234567890"
AUTH = (USER, PASSWORD)
driver = GraphDatabase.driver(URI, auth=AUTH)

try:
    driver.verify_connectivity()
    print("✅ Connection to Neo4j successful!")
except Exception as e:
    print(f"❌ Connection failed: {e}")
    exit(1)

# Load interests (artists/albums/songs)
print("\n📥 Loading interests...")
with open("../raw/interests.csv", "r", encoding="utf-8") as f:
    reader = csv.DictReader(f)
    for row in reader:
        driver.execute_query(
            """
            CREATE (i:Interest {
                name: $name,
                type: $type,
                description: $description
            })
            """,
            {
                "name": row["name"],
                "type": row["type"],
                "description": row["description"],
            },
        )
print("✅ Interests loaded!")

# Load hierarchical relationships (songs -> albums)
print("\n📥 Loading interest relationships...")
with open("../raw/subinterest.csv", "r", encoding="utf-8") as f:
    reader = csv.DictReader(f)
    for row in reader:
        driver.execute_query(
            """
            MATCH (child:Interest {name: $child_name})
            MATCH (parent:Interest {name: $parent_name})
            CREATE (child)-[:HAS_SUBINTEREST]->(parent)
            """,
            {
                "child_name": row["child_interest"],
                "parent_name": row["parent_interest"],
            },
        )
print("✅ Interest relationships loaded!")

# Load genre relationships
print("\n📥 Loading genre relationships...")
with open("../raw/has_genre.csv", "r", encoding="utf-8") as f:
    reader = csv.DictReader(f)
    genres_created = set()

    for row in reader:
        genre_name = row["genre_name"]

        # Create genre node if it doesn't exist
        if genre_name not in genres_created:
            driver.execute_query(
                """
                MERGE (g:Genre {name: $genre_name})
                """,
                {"genre_name": genre_name},
            )
            genres_created.add(genre_name)

        # Create relationship between interest and genre
        driver.execute_query(
            """
            MATCH (i:Interest {name: $interest_name})
            MATCH (g:Genre {name: $genre_name})
            CREATE (i)-[:HAS_GENRE]->(g)
            """,
            {"interest_name": row["interest_name"], "genre_name": genre_name},
        )
print("✅ Genre relationships loaded!")

# Print summary
summary, _, _ = driver.execute_query(
    """
    MATCH (i:Interest)
    WITH count(i) as interests
    MATCH (g:Genre)
    WITH interests, count(g) as genres
    MATCH ()-[r:BELONGS_TO]->()
    WITH interests, genres, count(r) as hierarchies
    MATCH ()-[r2:HAS_GENRE]->()
    RETURN interests, genres, hierarchies, count(r2) as genre_relations
    """
)

if summary:
    stats = summary[0]
    print(f"\n📊 Summary:")
    print(f"   • Interests: {stats['interests']}")
    print(f"   • Genres: {stats['genres']}")
    print(f"   • Hierarchical relationships: {stats['hierarchies']}")
    print(f"   • Genre relationships: {stats['genre_relations']}")

driver.close()
print("\n✅ All data loaded successfully!")
