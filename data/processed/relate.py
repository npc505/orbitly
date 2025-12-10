from neo4j import GraphDatabase
import random

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


interests, _, _ = driver.execute_query("MATCH (i:Interest) RETURN ID(i) AS interest_id")
users, _, _ = driver.execute_query("MATCH (u:User) RETURN u.username AS username")

gustos_lol = {}

for user in users:
    gustos = {random.randint(0, 5847) for _ in range(15)}
    gustos_lol[user.get("username")] = gustos

    for gusto in gustos:
        driver.execute_query(
            "MATCH (i:Interest), (u:User { username: $username }) WHERE ID(i) = $interest_id \
             CREATE (u)-[l:LIKES]->(i) SET l.created_on = datetime({ year: $year, month: $month, day: $day, hour: $hour, minute: $minute, second: $second })",
            {
                "username": user.get("username"),
                "interest_id": gusto,
                "day": random.randint(1, 28),
                "month": random.randint(1, 12),
                "year": random.randint(2024, 2025),
                "hour": random.randint(1, 23),
                "minute": random.randint(1, 59),
                "second": random.randint(1, 59),
            },
        )

for user in {random.choice(list(gustos_lol.keys())) for _ in range(250)}:
    for _try in range(5):
        try:
            target = random.choice(list(gustos_lol.keys()))

            print(user, target)
            gustos_compartidos = random.choices(list(gustos_lol[user]), k=5)

            if random.randint(0, 10) % 2 == 0:
                driver.execute_query(
                    "MATCH (u1:User{ username: $username1 }), (u2:User{ username: $username2 }) \
                     CREATE (u1)-[:MATCHES]->(u2)",
                    {
                        "username1": target,
                        "username2": user,
                    },
                )

            for gusto in gustos_compartidos:
                driver.execute_query(
                    "MATCH (i:Interest), (u:User { username: $username }) WHERE ID(i) = $interest_id \
                     CREATE (u)-[l:LIKES]->(i) SET l.created_on = datetime({ year: $year, month: $month, day: $day,  hour: $hour, minute: $minute, second: $second })",
                    {
                        "username": target,
                        "interest_id": gusto,
                        "day": random.randint(1, 28),
                        "month": random.randint(1, 12),
                        "year": random.randint(2024, 2025),
                        "hour": random.randint(1, 23),
                        "minute": random.randint(1, 59),
                        "second": random.randint(1, 59),
                    },
                )

            break
        except KeyError:
            pass


driver.close()
