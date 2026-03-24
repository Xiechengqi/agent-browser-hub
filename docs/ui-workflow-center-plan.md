# UI Workflow Center Plan

## Goal

Shift the web UI from a flat command index to a workflow-centered control surface.

## Primary Views

- Workflow Overview
- Site Detail
- Sources
- Health

## Overview Card Data

- site
- display name
- source type: builtin / external
- package version
- health
- command count
- auth strategy
- package kind

## Site Detail

- command list
- params and execution metadata
- source location
- override state
- debug entry points

## Sources View

- configured external sources
- current resolution result
- fallback status
- version compatibility issues

## Required API Data

- workflow source label
- workflow origin kind
- workflow origin location
- fallback active state
- package version
- auth strategy
- package kind
- command count

## Rollout Stages

### Stage 1

- keep current command list
- add source badges and workflow origin fields
- expose configured workflow settings

### Stage 2

- add package overview cards by site
- add site detail drawer/page
- show builtin versus external effective state

### Stage 3

- add source health dashboard
- add refresh/diagnostic actions for external sources
- add package-level migration status indicators

## Design Direction

Use the Corporate Trust system from `CLAUDE.md`:

- centralized color and shadow tokens
- elevated cards
- clear source badges
- strong site hierarchy
- responsive split-pane layouts on desktop
