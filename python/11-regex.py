import re

phone_num_regex = re.compile(r'(\d\d\d)-(\d\d\d-\d\d\d\d)')

mo = phone_num_regex.search('My number is 415-555-4242.')
print(mo)
print(f'Phone number found: {mo.group(0)}')
print(mo.group(1))
print(mo.group(2))
print(mo.groups())

mo = phone_num_regex.search("24323434234242432342")
print(mo) # None when no match

# findall() - find all matches

# flags:
# re.DOTALL - make dot match all chars, including newline
# re.IGNORECASE - case insensitive
# re.VERBOSE - i think this makes the re engine ignore whitespace as well as allowing comments

names_regex = re.compile(r'Agent \w+')
redacted = names_regex.sub('CENSORED', 'Agent Alice gave the secret documents to Agent Bob.')
print(redacted)