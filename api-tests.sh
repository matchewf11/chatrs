#!/bin/bash

fail() {
	echo "Fail: $1" >&2
	exit 1
}

assert_eq() {
	[[ "$1" == "$2" ]] || fail "$1 != $2"
}

assert_neq() {
	[[ "$1" != "$2" ]] || fail "$1 == $2"
}

username="user_$(date +%s)"
password="foobar"
room="room_$(date +%s)"

echo "Testing [POST] /users"
code=$(curl -s -o /dev/null -w "%{http_code}" \
	-X POST http://localhost:3000/users \
	-H "Content-Type: application/json" \
	-d "{ \"username\": \"$username\", \"password\": \"$password\" }")
assert_eq "$code" "200"

echo "Testing [POST] /sessions"
token=$(curl -s \
	-X POST "http://localhost:3000/sessions" \
	-H "Content-Type: application/json" \
	-d "{ \"username\": \"$username\", \"password\": \"$password\" }" |
	jq -r ".token")
assert_neq "$token" ""

echo "Testing [POST] /sessions w/o Auth"
code=$(curl -s -w "%{http_code}" -X POST "http://localhost:3000/rooms")
assert_eq "$code" "401"

echo "TESTING [POST] /rooms"
code=$(curl -s -o /dev/null -w "%{http_code}" -X POST "http://localhost:3000/rooms" \
	-H "Authorization: Bearer $token" \
	-H "Content-Type: application/json" \
	-d "{ \"name\": \"$room\" }")
assert_eq "$code" "200"




echo "TESTING [POST] /chats"
assert_eq "1" "0"

# test chats

echo "All Tests Passed"
