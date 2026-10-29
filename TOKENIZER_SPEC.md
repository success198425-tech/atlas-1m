# Atlas Tokenizer Specification v0.1

## Objective

Convert text into token IDs.

Example:

Input:
Motor vibration increased

Output:
[4, 5, 6]

## Reserved Tokens

0 = <PAD>
1 = <UNK>
2 = <BOS>
3 = <EOS>

## Rules

- Lowercase text
- Split by whitespace
- Remove punctuation
- Build vocabulary from Atlas datasets

## Dataset Sources

- RCA Reports
- Maintenance Records
- PLC Logs
- SCADA Events
- Quality Reports

## Future Compatibility

Atlas-1M
Atlas-10M
Atlas-100M
Atlas Foundation
