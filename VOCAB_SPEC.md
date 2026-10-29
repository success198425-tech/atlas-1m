# Atlas Vocabulary Specification v0.1

## Reserved Tokens

0 = <PAD>
1 = <UNK>
2 = <BOS>
3 = <EOS>

## Vocabulary Size

2048

## Tokenization Rules

- Convert text to lowercase
- Remove punctuation
- Split by whitespace
- Unknown words become <UNK>

## Example

Input:

Motor vibration increased

Output Tokens:

motor
vibration
increased

Output IDs:

[4, 5, 6]

## Future Support

- Manufacturing terminology
- PLC terminology
- SCADA terminology
- RCA terminology
