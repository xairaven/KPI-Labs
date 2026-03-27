package ua.kpi.lab6

import android.os.Bundle
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity

class DetailActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_detail)

        val tvDetailTheme = findViewById<TextView>(R.id.tvDetailTheme)
        val tvDetailMessage = findViewById<TextView>(R.id.tvDetailMessage)

        // Extract data passed from the notification intent
        val theme = intent.getStringExtra("theme") ?: "Без теми"
        val message = intent.getStringExtra("message") ?: "Немає тексту"

        tvDetailTheme.text = theme
        tvDetailMessage.text = message
    }
}