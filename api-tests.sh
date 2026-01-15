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

base_url="http://localhost:3000"
username="user_$(date +%s)"
password="foobar"
room="room_$(date +%s)"

echo "Testing [POST] /users"
code=$(curl -s -o /dev/null -w "%{http_code}" \
	-X POST "$base_url/users" \
	-H "Content-Type: application/json" \
	-d "{ \"username\": \"$username\", \"password\": \"$password\" }")
assert_eq "$code" "200"

echo "Testing [POST] /sessions"
token=$(curl -s \
	-X POST "$base_url/sessions" \
	-H "Content-Type: application/json" \
	-d "{ \"username\": \"$username\", \"password\": \"$password\" }" |
	jq -r ".token")
assert_neq "$token" ""

echo "Testing [POST] /sessions w/o Auth"
code=$(curl -s -w "%{http_code}" -X POST "$base_url/rooms")
assert_eq "$code" "401"

echo "TESTING [POST] /rooms"
code=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$base_url/rooms" \
	-H "Authorization: Bearer $token" \
	-H "Content-Type: application/json" \
	-d "{ \"name\": \"$room\" }")
assert_eq "$code" "200"

# this assumes room above is 1
echo "TESTING [POST] /rooms/{room_id}"
code=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$base_url/rooms/1" \
	-H "Authorization: Bearer $token")
assert_eq "$code" "200"

echo "TESTING [POST] /chats"
code=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$base_url/chats" \
	-H "Authorization: Bearer $token" \
	-H "Content-Type: application/json" \
	-d "{ \"room\": \"$room\", \"body\": \"Hello\"}")
assert_eq "$code" "200"

echo "All Tests Passed"
