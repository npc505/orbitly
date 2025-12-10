from neo4j import GraphDatabase
import pandas as pd


def load_movies_and_genres_to_neo4j(df, uri, user, password):
    """
    Inserta nodos :Movie, :Genre y relaciones :HASGENRE
    en Neo4j usando el primer género de cada película.
    """

    # 1. Preparamos las filas: título y primer género
    rows = []
    for title, genres in df.iterrows():
        if title and genres:
            rows.append({"title": str(title), "genre": str(genres)})

    if not rows:
        print("No hay filas válidas con título y género.")
        return

    # 2. Conectamos a Neo4j
    driver = GraphDatabase.driver(uri, auth=(user, password))

    cypher = """
    UNWIND $rows AS row
    MERGE (m:Interest:Game {name: row.name, type: "Videojuego"})
    MERGE (g:Genre {name: row.genre})
    MERGE (m)-[:HAS_GENRE]->(g)
    """

    # 3. Ejecutamos el Cypher en una sesión
    with driver.session() as session:
        session.run(cypher, rows=rows)

    driver.close()
    print(f"Insertadas/actualizadas {len(rows)} películas y sus géneros.")


df = pd.read_csv("../raw/movies_genres.csv")

load_movies_and_genres_to_neo4j(df, "bolt://127.0.0.1:7687", "neo4j", "1234567890")
