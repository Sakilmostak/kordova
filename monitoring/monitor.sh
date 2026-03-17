#!/bin/bash

# Library Management System - Continuous Monitoring Script
# Tracks agent progress, task completion, and system health

TIMESTAMP=$(date)
MONITORING_DIR="/home/reghoul/.openclaw/shared-project/library-management-system/monitoring"
MESSAGES_DIR="/home/reghoul/.openclaw/shared-project/messages"

echo "=== LMS PROJECT MONITORING REPORT ===" > "$MONITORING_DIR/latest_status.txt"
echo "Time: $TIMESTAMP" >> "$MONITORING_DIR/latest_status.txt"
echo "" >> "$MONITORING_DIR/latest_status.txt"

# Check each agent's progress
echo "📋 Agent Progress:" >> "$MONITORING_DIR/latest_status.txt"
for agent in researcher architect planner developer tester reviewer documenter; do
    inbox=$(find $MESSAGES_DIR/$agent/inbox -type f 2>/dev/null | wc -l)
    processing=$(find $MESSAGES_DIR/$agent/processing -type f 2>/dev/null | wc -l)
    completed=$(find $MESSAGES_DIR/$agent/completed -type f 2>/dev/null | wc -l)
    failed=$(find $MESSAGES_DIR/$agent/dead_letter -type f 2>/dev/null | wc -l)
    
    status="⏳ Queued"
    if [ $processing -gt 0 ]; then
        status="🟡 Processing"
    elif [ $completed -gt 0 ]; then
        status="✅ Completed"
    elif [ $failed -gt 0 ]; then
        status="❌ Failed"
    fi
    
    echo "$agent: $status (I:$inbox P:$processing C:$completed F:$failed)" >> "$MONITORING_DIR/latest_status.txt"
done

# Check project directory for deliverables
echo "" >> "$MONITORING_DIR/latest_status.txt"
echo "📁 Project Deliverables:" >> "$MONITORING_DIR/latest_status.txt"
find "/home/reghoul/.openclaw/shared-project/library-management-system" -name "*.md" -o -name "*.rs" -o -name "*.toml" -o -name "*.json" | wc -l | xargs echo "Files created:" >> "$MONITORING_DIR/latest_status.txt"

echo "Monitoring snapshot saved to: $MONITORING_DIR/latest_status.txt"