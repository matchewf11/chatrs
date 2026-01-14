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
curl -X POST "http://localhost:3000/rooms"

echo "Testing POST /rooms with auth"
curl -X POST "http://localhost:3000/rooms" \
	-H "Authorization: Bearer EFJXEUagNjvnysD5hz5l" \
	-H "Content-Type: application/json" \
	-d '{ "name": "room_test" }'
