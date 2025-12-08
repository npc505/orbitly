"""
Created with Gemini 3 Pro
"""

import csv
import random
from faker import Faker
import sys


fake = Faker()

NUM_USERS = 200
target_path = sys.argv[1]

headers = [
    "Mail",
    "Password",
    "Username",
    "First Name",
    "Last Name",
    "Country",
    "Description",
    "Profile Picture",
]

print(f"Generating {NUM_USERS} users...")

with open(target_path, mode="w", newline="", encoding="utf-8") as file:
    writer = csv.writer(file)
    writer.writerow(headers)

    for _ in range(NUM_USERS):
        # Generate basic info
        first_name = fake.first_name()
        last_name = fake.last_name()
        username = f"{first_name.lower()}{last_name.lower()}{random.randint(1, 99)}"
        email = f"{username}@example.com"

        # Generate a unique profile picture URL using the email as a seed
        # This ensures the image stays consistent for that user
        pfp_url = f"https://i.pravatar.cc/150?u={email}"

        row = [
            email,  # Mail
            fake.password(length=12),  # Password
            username,  # Username
            first_name,  # First Name
            last_name,  # Last Name
            fake.country(),  # Country
            fake.sentence(nb_words=10),  # Description
            pfp_url,  # Profile Picture
        ]

        writer.writerow(row)

print(f"Done! Wrote in {target_path}")
