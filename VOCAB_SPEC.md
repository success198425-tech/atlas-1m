# Atlas Vocabulary Specification v0.1

## Vocabulary Size

2048

## Reserved Tokens

0 = <PAD>
1 = <UNK>
2 = <BOS>
3 = <EOS>

## Tokenization Rules

1. Convert text to lowercase
2. Remove punctuation
3. Split by whitespace
4. Keep engineering terms intact
5. Unknown words → <UNK>

## Engineering Terms

Examples:

plc
scada
servo
bearing
vibration
temperature
current
alarm
fault
maintenance

## Example

Input:

Motor vibration increased

Output Tokens:

motor
vibration
increased

Output IDs:

[4,5,6]
