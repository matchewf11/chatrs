#!/bin/bash

echo "Testing POST /users"
curl -X POST "http://localhost:3000/users" \
	-H "Content-Type: application/json" \
	-d '{ "username": "matchew", "password": "foobar" }'

# echo "Testing GET /chats"
# curl "http://localhost:3000/chats" | jq
#
# echo "Testing POST /chats"
# curl -X POST "http://localhost:3000/chats" \
# 	-H "Content-Type: application/json" \
# 	-d '{ "body": "This is my first chat posted." }' | jq
#
