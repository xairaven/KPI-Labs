package ua.kpi.lab4

import android.content.Intent
import android.net.ConnectivityManager
import android.net.NetworkCapabilities
import android.os.Bundle
import android.widget.Button
import android.widget.TextView
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import androidx.lifecycle.lifecycleScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.delay
import kotlinx.coroutines.isActive
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import org.json.JSONObject
import java.io.BufferedReader
import java.io.InputStreamReader
import java.net.HttpURLConnection
import java.net.URL
import java.text.SimpleDateFormat
import java.util.Date
import java.util.Locale

class MainActivity : AppCompatActivity() {

    private lateinit var dbHelper: DatabaseHelper
    private lateinit var tvCurrentStatus: TextView

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        dbHelper = DatabaseHelper(this)
        tvCurrentStatus = findViewById(R.id.tvCurrentStatus)
        val btnViewHistory = findViewById<Button>(R.id.btnViewHistory)

        btnViewHistory.setOnClickListener {
            startActivity(Intent(this, HistoryActivity::class.java))
        }

        // Check internet connection on startup
        if (isNetworkAvailable()) {
            Toast.makeText(this, "Онлайн режим. Опитування сервера...", Toast.LENGTH_SHORT).show()
            startPollingServer()
        } else {
            Toast.makeText(
                this,
                "Автономний режим. Доступний лише перегляд бази.",
                Toast.LENGTH_LONG
            ).show()
            tvCurrentStatus.text = "Немає підключення до мережі"
        }
    }

    // Helper method to check internet connectivity
    private fun isNetworkAvailable(): Boolean {
        val connectivityManager = getSystemService(CONNECTIVITY_SERVICE) as ConnectivityManager
        val network = connectivityManager.activeNetwork ?: return false
        val activeNetwork = connectivityManager.getNetworkCapabilities(network) ?: return false
        return activeNetwork.hasCapability(NetworkCapabilities.NET_CAPABILITY_INTERNET)
    }

    // Coroutine to poll the server every 20 seconds
    private fun startPollingServer() {
        lifecycleScope.launch(Dispatchers.IO) {
            while (isActive) {
                if (isNetworkAvailable()) {
                    fetchCurrentSong()
                }
                // Delay for 20 seconds
                delay(20000)
            }
        }
    }

    // Fetch data from API and process it
    private suspend fun fetchCurrentSong() {
        try {
            val url = URL("https://webradio.io/api/radio/pi/current-song")
            val connection = url.openConnection() as HttpURLConnection
            connection.requestMethod = "GET"
            connection.connectTimeout = 5000

            if (connection.responseCode == HttpURLConnection.HTTP_OK) {
                val reader = BufferedReader(InputStreamReader(connection.inputStream))
                val response = StringBuilder()
                var line: String?
                while (reader.readLine().also { line = it } != null) {
                    response.append(line)
                }
                reader.close()

                parseAndSaveData(response.toString())
            }
            connection.disconnect()
        } catch (e: Exception) {
            e.printStackTrace()
            updateUIStatus("Помилка з'єднання з сервером")
        }
    }

    // Parse JSON and interact with database
    private suspend fun parseAndSaveData(jsonString: String) {
        try {
            val jsonObject = JSONObject(jsonString)

            // Fetching data
            val artist = jsonObject.optString("artist", "Невідомий виконавець")
            val track = jsonObject.optString("title", "Невідомий трек")

            updateUIStatus("Зараз грає:\n$artist - $track")

            val lastTrackInDb = dbHelper.getLastTrackName()
            if (lastTrackInDb != track) {
                val currentTime =
                    SimpleDateFormat("yyyy-MM-dd HH:mm:ss", Locale.getDefault()).format(Date())
                dbHelper.insertSong(artist, track, currentTime)
            }

        } catch (e: Exception) {
            e.printStackTrace()
            updateUIStatus("Помилка парсингу: ${e.message}\nСирі дані:\n$jsonString")
        }
    }

    // Helper to update TextView from background threads
    private suspend fun updateUIStatus(message: String) {
        withContext(Dispatchers.Main) {
            tvCurrentStatus.text = message
        }
    }
}