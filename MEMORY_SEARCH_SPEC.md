# Atlas Memory Search Specification v0.1

## Goal

Retrieve relevant engineering memories based on keywords.

## Search Process

User Query
↓
Keyword Extraction
↓
Memory Index Lookup
↓
Memory Retrieval
↓
Rank Results
↓
Return Matching Records

## Example

Query:

bearing vibration

Result:

RCA-0001
MAINT-0001

## Search Fields

- problem
- observation
- hypothesis
- action
- result
- lesson
- keywords

## Ranking Rules

Exact match = High score

Partial match = Medium score

Related term = Low score

## Output Example

Query:

network timeout

Results:

PLC-0001
RCA-0002
