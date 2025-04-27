#!/bin/bash

# push-all.sh - Script to push changes to both origin and upstream repositories
# Created by Augment Agent

# Set colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Define user configurations
declare -A USER1=(
    [name]="garveyshah"
    [email]="garveyshah1595@gmail.com"
)

declare -A USER2=(
    [name]="oumaoumag"
    [email]="ouma.godwin10@gmail.com"
)

# Function to switch Git user
switch_git_user() {
    local name=$1
    local email=$2
    
    git config user.name "$name"
    git config user.email "$email"
    echo -e "${GREEN}Switched to user: $name ($email)${NC}"
}

echo -e "${YELLOW}Pushing to multiple Git remotes...${NC}"

# Get current branch
CURRENT_BRANCH=$(git symbolic-ref --short HEAD)
if [ $? -ne 0 ]; then
    echo -e "${RED}Error: Failed to determine current branch.${NC}"
    exit 1
fi

echo -e "Current branch: ${GREEN}$CURRENT_BRANCH${NC}"

# Push as first user
echo -e "\n${YELLOW}Pushing as ${USER1[name]}...${NC}"
switch_git_user "${USER1[name]}" "${USER1[email]}"
git push origin $CURRENT_BRANCH
if [ $? -ne 0 ]; then
    echo -e "${RED}Error: Failed to push to origin as ${USER1[name]}.${NC}"
    echo -e "${YELLOW}You may need to run: git push -u origin $CURRENT_BRANCH${NC}"
    exit 1
fi
echo -e "${GREEN}Successfully pushed to origin as ${USER1[name]}.${NC}"

# Push as second user
echo -e "\n${YELLOW}Pushing as ${USER2[name]}...${NC}"
switch_git_user "${USER2[name]}" "${USER2[email]}"
git push upstream $CURRENT_BRANCH
if [ $? -ne 0 ]; then
    echo -e "${RED}Error: Failed to push to upstream as ${USER2[name]}.${NC}"
    echo -e "${YELLOW}You may need to run: git push -u upstream $CURRENT_BRANCH${NC}"
    echo -e "${YELLOW}Or you may not have write access to the upstream repository.${NC}"
    exit 1
fi
echo -e "${GREEN}Successfully pushed to upstream as ${USER2[name]}.${NC}"

echo -e "\n${GREEN}All pushes completed successfully!${NC}"