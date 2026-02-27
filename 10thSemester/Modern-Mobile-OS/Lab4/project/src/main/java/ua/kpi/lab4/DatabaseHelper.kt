package ua.kpi.lab4

import android.content.ContentValues
import android.content.Context
import android.database.sqlite.SQLiteDatabase
import android.database.sqlite.SQLiteOpenHelper

// Helper class for managing WebRadio database
class DatabaseHelper(context: Context) :
    SQLiteOpenHelper(context, DATABASE_NAME, null, DATABASE_VERSION) {

    companion object {
        private const val DATABASE_NAME = "webradio.db"
        private const val DATABASE_VERSION = 1

        private const val TABLE_NAME = "songs_history"
        private const val COLUMN_ID = "id"
        private const val COLUMN_ARTIST = "artist"
        private const val COLUMN_TRACK = "track_name"
        private const val COLUMN_TIME = "time_added"
    }

    override fun onCreate(db: SQLiteDatabase) {
        // Create table for storing song statistics
        val createTableQuery = ("CREATE TABLE $TABLE_NAME ("
                + "$COLUMN_ID INTEGER PRIMARY KEY AUTOINCREMENT, "
                + "$COLUMN_ARTIST TEXT, "
                + "$COLUMN_TRACK TEXT, "
                + "$COLUMN_TIME TEXT)")
        db.execSQL(createTableQuery)
    }

    override fun onUpgrade(db: SQLiteDatabase, oldVersion: Int, newVersion: Int) {
        db.execSQL("DROP TABLE IF EXISTS $TABLE_NAME")
        onCreate(db)
    }

    // Insert a new song record into the database
    fun insertSong(artist: String, track: String, time: String) {
        val db = this.writableDatabase
        val values = ContentValues()
        values.put(COLUMN_ARTIST, artist)
        values.put(COLUMN_TRACK, track)
        values.put(COLUMN_TIME, time)
        db.insert(TABLE_NAME, null, values)
        db.close()
    }

    // Retrieve the name of the last saved track to prevent duplicates
    fun getLastTrackName(): String? {
        val db = this.readableDatabase
        var lastTrack: String? = null
        val query = "SELECT $COLUMN_TRACK FROM $TABLE_NAME ORDER BY $COLUMN_ID DESC LIMIT 1"
        val cursor = db.rawQuery(query, null)

        if (cursor.moveToFirst()) {
            lastTrack = cursor.getString(0)
        }
        cursor.close()
        db.close()
        return lastTrack
    }

    // Retrieve all saved songs as a formatted string
    fun getAllSongs(): String {
        val db = this.readableDatabase
        val cursor = db.rawQuery("SELECT * FROM $TABLE_NAME ORDER BY $COLUMN_ID DESC", null)
        val stringBuilder = StringBuilder()

        if (cursor.moveToFirst()) {
            do {
                val artist = cursor.getString(cursor.getColumnIndexOrThrow(COLUMN_ARTIST))
                val track = cursor.getString(cursor.getColumnIndexOrThrow(COLUMN_TRACK))
                val time = cursor.getString(cursor.getColumnIndexOrThrow(COLUMN_TIME))
                stringBuilder.append("Час: $time\nВиконавець: $artist\nТрек: $track\n\n")
            } while (cursor.moveToNext())
        } else {
            stringBuilder.append("Історія порожня.")
        }
        cursor.close()
        db.close()
        return stringBuilder.toString()
    }
}