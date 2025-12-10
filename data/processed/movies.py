import pandas as pd
import numpy as np
import json

df = pd.read_csv("../raw/movies_metadata.csv/movies_metadata.csv")

import ast
import numpy as np


def to_dict_safe(x):
    try:
        return ast.literal_eval(x) if isinstance(x, str) else np.nan
    except:
        return np.nan


df["genres"] = df["genres"].apply(to_dict_safe)

df["title"]

df["genres"]


def escape_quotes(s: str) -> str:
    if not isinstance(s, str):
        return s
    return s.replace("'", "\\'")


from neo4j import GraphDatabase


def extract_genre_pairs(
    df: pd.DataFrame, title_col: str = "title", genres_col: str = "genres"
):
    """
    Devuelve una lista de dicts:
    [{ "title": ..., "genre": ... }, ...]
    con TODOS los géneros de cada película.
    """
    pairs = []

    for _, row in df.iterrows():
        title = row[title_col]
        genre_list = row[genres_col]

        if not isinstance(genre_list, list):
            continue

        for g in genre_list:
            if isinstance(g, dict):
                genre_name = g.get("name")
                if genre_name:
                    pairs.append(
                        {
                            "title": str(title),
                            "genre": str(genre_name),
                        }
                    )

    return pairs


CYPHER_MOVIE_GENRE_CATEGORY = """
UNWIND $rows AS row
MERGE (m:Interest {name: row.title})
MERGE (c:Category {name: "movie"})
MERGE (m)-[:BELONGS_TO]->(c)
MERGE (g:Genre {name: row.genre, description: row.genre})
MERGE (m)-[:HAS_GENRE]->(g)
"""

# Conecta a tu instancia
driver = GraphDatabase.driver("bolt://127.0.0.1:7687", auth=("neo4j", "1234567890"))


def load_movies_genres_and_category(
    df: pd.DataFrame,
    driver,
    title_col: str = "title",
    genres_col: str = "genres",
):
    rows = extract_genre_pairs(df, title_col=title_col, genres_col=genres_col)
    print(f"Loaded {len(rows)} rows")

    with driver.session() as session:
        session.run(CYPHER_MOVIE_GENRE_CATEGORY, rows=rows)


load_movies_genres_and_category(df.head(2500), driver)

driver.close()
