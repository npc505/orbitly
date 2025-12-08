CREATE CONSTRAINT user_username_unique IF NOT EXISTS FOR (u:User) REQUIRE u.username IS UNIQUE;
CREATE CONSTRAINT user_mail_unique IF NOT EXISTS FOR (u:User) REQUIRE u.mail IS UNIQUE; // Jaja
