#!/bin/bash

# Change device ownership
sudo chown -R $REMOTE_USER:$REMOTE_GROUP $DEVICE
# resolves a npm bug related to these being set to root
sudo chown -R vscode  "/home/vscode/.npm"
