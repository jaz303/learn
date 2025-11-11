with open("NOTES.md") as file:
    notes = file.read()

print(notes)

with open("NOTES.md") as file:
    note_lines = file.readlines()
    
print(note_lines)

with open("NOTES.md") as file:
    for line in file:
        print(line, end='')
        
with open("tmp.txt", "w") as out:
    out.write("bacon is not a vegetable")

