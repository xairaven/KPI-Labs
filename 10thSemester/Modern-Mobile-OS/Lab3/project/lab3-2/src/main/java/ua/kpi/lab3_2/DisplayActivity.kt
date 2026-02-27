package ua.kpi.lab3_2

import android.os.Bundle
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity

class DisplayActivity : AppCompatActivity() {

    private lateinit var dbHelper: DatabaseHelper

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_display)

        dbHelper = DatabaseHelper(this)

        val tvDatabaseContent = findViewById<TextView>(R.id.tvDatabaseContent)

        // Fetch and display all data from the database
        val allData = dbHelper.getAllStudents()
        tvDatabaseContent.text = allData
    }
}