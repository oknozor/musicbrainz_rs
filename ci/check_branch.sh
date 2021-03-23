#!/bin/sh

branch=$(git rev-parse --abbrev-ref HEAD)

if [ $branch != "main" ]
then
    echo "Needs to be on main branch to bump current version"
    exit 1
fi
