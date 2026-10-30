# Atlas Memory Object Specification

## Purpose

Defines the standard memory structure used throughout Atlas.

---

## Required Fields

- id
- title
- category
- tags
- problem
- root_cause
- solution
- confidence

---

## Optional Fields

- references
- related_memories
- notes

---

## Example

id: RCA-0001

title: Conveyor Sudden Stop

category: RCA

tags:
  - conveyor
  - motor
  - bearing

problem:
  Conveyor unexpectedly stopped during operation.

root_cause:
  Bearing wear caused motor overload.

solution:
  Replace bearing and inspect lubrication schedule.

confidence: 95

related_memories:
  - MAINT-0001
  - SCADA-0001
  - PLC-0001
