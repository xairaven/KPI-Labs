package ua.kpi.lab3_2

import android.content.ContentValues
import android.content.Context
import android.database.sqlite.SQLiteDatabase
import android.database.sqlite.SQLiteOpenHelper

// Helper class for managing SQLite database with updated schema
class DatabaseHelper(context: Context) :
    SQLiteOpenHelper(context, DATABASE_NAME, null, DATABASE_VERSION) {
    companion object {
        private const val DATABASE_NAME = "university.db"

        // Changed version to 2 to trigger onUpgrade method
        private const val DATABASE_VERSION = 2

        // Table and new columns names
        private const val TABLE_NAME = "classmates"
        private const val COLUMN_ID = "id"
        private const val COLUMN_LAST_NAME = "last_name"
        private const val COLUMN_FIRST_NAME = "first_name"
        private const val COLUMN_PATRONYMIC = "patronymic"
        private const val COLUMN_TIME = "time_added"
    }

    override fun onCreate(db: SQLiteDatabase) {
        // Create table query with new separated name fields
        val createTableQuery = ("CREATE TABLE $TABLE_NAME ("
                + "$COLUMN_ID INTEGER PRIMARY KEY AUTOINCREMENT, "
                + "$COLUMN_LAST_NAME TEXT, "
                + "$COLUMN_FIRST_NAME TEXT, "
                + "$COLUMN_PATRONYMIC TEXT, "
                + "$COLUMN_TIME TEXT)")
        db.execSQL(createTableQuery)
    }

    override fun onUpgrade(db: SQLiteDatabase, oldVersion: Int, newVersion: Int) {
        // Drop older table if exists and create fresh with new structure
        db.execSQL("DROP TABLE IF EXISTS $TABLE_NAME")
        onCreate(db)
    }

    // Delete all records from the table
    fun clearDatabase() {
        val db = this.writableDatabase
        db.execSQL("DELETE FROM $TABLE_NAME")
        db.close()
    }

    // Insert a new student record with separated name parts
    fun addStudent(lastName: String, firstName: String, patronymic: String, time: String) {
        val db = this.writableDatabase
        val values = ContentValues()
        values.put(COLUMN_LAST_NAME, lastName)
        values.put(COLUMN_FIRST_NAME, firstName)
        values.put(COLUMN_PATRONYMIC, patronymic)
        values.put(COLUMN_TIME, time)
        db.insert(TABLE_NAME, null, values)
        db.close()
    }

    // Update the last added record's separated name fields
    fun updateLastStudent(lastName: String, firstName: String, patronymic: String) {
        val db = this.writableDatabase
        // Update where ID is the maximum ID in the table
        val updateQuery = ("UPDATE $TABLE_NAME SET "
                + "$COLUMN_LAST_NAME = '$lastName', "
                + "$COLUMN_FIRST_NAME = '$firstName', "
                + "$COLUMN_PATRONYMIC = '$patronymic' "
                + "WHERE $COLUMN_ID = (SELECT MAX($COLUMN_ID) FROM $TABLE_NAME)")
        db.execSQL(updateQuery)
        db.close()
    }

    // Retrieve all students as a formatted string with new fields
    fun getAllStudents(): String {
        val db = this.readableDatabase
        val cursor = db.rawQuery("SELECT * FROM $TABLE_NAME", null)
        val stringBuilder = StringBuilder()

        if (cursor.moveToFirst()) {
            do {
                val id = cursor.getInt(cursor.getColumnIndexOrThrow(COLUMN_ID))
                val lastName = cursor.getString(cursor.getColumnIndexOrThrow(COLUMN_LAST_NAME))
                val firstName = cursor.getString(cursor.getColumnIndexOrThrow(COLUMN_FIRST_NAME))
                val patronymic = cursor.getString(cursor.getColumnIndexOrThrow(COLUMN_PATRONYMIC))
                val time = cursor.getString(cursor.getColumnIndexOrThrow(COLUMN_TIME))

                stringBuilder.append("ID: $id\n")
                stringBuilder.append("Прізвище: $lastName\n")
                stringBuilder.append("Ім'я: $firstName\n")
                stringBuilder.append("По-батькові: $patronymic\n")
                stringBuilder.append("Час: $time\n\n")
            } while (cursor.moveToNext())
        }
        cursor.close()
        db.close()

        return stringBuilder.toString()
    }
}