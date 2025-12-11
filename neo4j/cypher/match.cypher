// CREATE CONSTRAINT match_relation_unique IF NOT EXISTS FOR (u1:User)-[r:MATCHES]->(u2:User) REQUIRE [u1.username, u2.username] IS UNIQUE;
