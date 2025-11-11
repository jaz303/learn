from pathlib import Path

print(Path('usr'))
print(Path('usr').joinpath('bin').joinpath('ruby'))
print(Path('usr') / 'bin' / 'bash')

print(Path.cwd())

# Path(...).mkdir()
# Path(...).mkdir(parents=True)
# Path(...).is_absolute()
# Path(...).resolve()
# Path(...).relative_to("/")
# Path(...).exists()
# .is_file()
# .is_dir()
# .stat()
# .iterdir() - make an iterator for directory contents
# .unlink()
# .rmdir()
# .rglob()

# import shutil
# this deals with copying files, folders..., moving/renaming
# .rmtree(path)