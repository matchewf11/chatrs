#!/bin/bash

echo "Testing POST /users"
curl -X POST "http://localhost:3000/users" \
	-H "Content-Type: application/json" \
	-d '{ "username": "matchew", "password": "foobar" }'

echo "Testing POST /sessions"
curl -X POST "http://localhost:3000/sessions" \
	-H "Content-Type: application/json" \
	-d '{ "username": "matchew", "password": "foobar" }' | jq

echo "Testing POST /rooms without auth (should fail)"
curl -vX POST "http://localhost:3000/rooms"

echo "Testing POST /rooms with auth"
curl -vX POST "http://localhost:3000/rooms" \
	-H "Authorization: Bearer 0sHJ1PXzG4QzT7nU5M4M" \
	-H "Content-Type: application/json" \
	-d '{ "name": "room_test" }'
