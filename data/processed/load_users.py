from neo4j import GraphDatabase
import polars as pl


URI = "bolt://127.0.0.1:7687"
USER = "neo4j"
PASSWORD = "1234567890"
AUTH = (USER, PASSWORD)
driver = GraphDatabase.driver(URI, auth=(USER, PASSWORD))

try:
    driver.verify_connectivity()
    print("✅ Connection to Neo4j AuraDB successful!")
except Exception as e:
    print(f"❌ Connection failed: {e}")

df1 = pl.read_csv("../raw/users.csv")
df2 = pl.read_csv("../raw/users2.csv")


for user in df1.iter_rows(named=True):
    driver.execute_query(
        "CREATE (u:User { first_name: $first_name, last_name: $last_name, mail: $mail, description: $description, avatar: $profile_picture, username: $username })",
        **user,
    )

for user in df2.iter_rows(named=True):
    driver.execute_query(
        "CREATE (u:User { first_name: $first_name, last_name: $last_name, mail: $mail, username: $username })",
        **user,
    )

driver.close()
