import sqlite3
import os

def check_table_existence():
    home = os.environ["HOME"]
    conn = sqlite3.connect(f"{home}/.local/baker/index")
    cursor = conn.cursor()

    cursor.execute("SELECT id, baker_path, fs_path FROM files")
    result = cursor.fetchall()

    conn.close()

    if result:
        print(result)

check_table_existence()
