package ua.kpi.lab5

import android.content.ActivityNotFoundException
import android.content.Context
import android.content.Intent
import android.os.Bundle
import android.os.Environment
import android.view.Gravity
import android.view.LayoutInflater
import android.view.View
import android.widget.Button
import android.widget.CheckBox
import android.widget.EditText
import android.widget.PopupWindow
import android.widget.TextView
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import androidx.core.content.FileProvider
import androidx.lifecycle.lifecycleScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import java.io.File
import java.io.FileOutputStream
import java.net.HttpURLConnection
import java.net.URL

class MainActivity : AppCompatActivity() {

    private lateinit var etJournalId: EditText
    private lateinit var etArticleId: EditText
    private lateinit var etFileId: EditText
    private lateinit var tvStatus: TextView
    private lateinit var btnDownload: Button
    private lateinit var btnView: Button
    private lateinit var btnDelete: Button

    private var currentDownloadedFile: File? = null
    private val folderName = "JournalFiles"

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        etJournalId = findViewById(R.id.etJournalId)
        etArticleId = findViewById(R.id.etArticleId)
        etFileId = findViewById(R.id.etFileId)
        tvStatus = findViewById(R.id.tvStatus)
        btnDownload = findViewById(R.id.btnDownload)
        btnView = findViewById(R.id.btnView)
        btnDelete = findViewById(R.id.btnDelete)

        val storageDir = File(
            Environment.getExternalStoragePublicDirectory(Environment.DIRECTORY_DOCUMENTS),
            folderName
        )
        if (!storageDir.exists()) {
            storageDir.mkdirs()
        }

        val sharedPrefs = getSharedPreferences("AppPreferences", Context.MODE_PRIVATE)
        val showTutorial = sharedPrefs.getBoolean("show_tutorial", true)
        if (showTutorial) {
            findViewById<View>(android.R.id.content).post {
                showTutorialPopup()
            }
        }

        btnDownload.setOnClickListener {
            val journalId = etJournalId.text.toString().trim()
            val articleId = etArticleId.text.toString().trim()
            val fileId = etFileId.text.toString().trim()

            if (journalId.isNotEmpty() && articleId.isNotEmpty() && fileId.isNotEmpty()) {
                val fileUrl =
                    "https://chemengine.kpi.ua/article/download/$journalId/$articleId/$fileId"
                downloadPdfFile(fileUrl, storageDir)
            } else {
                Toast.makeText(this, "Please enter all three IDs", Toast.LENGTH_SHORT).show()
            }
        }

        btnView.setOnClickListener {
            currentDownloadedFile?.let { file ->
                openPdfFile(file)
            }
        }

        btnDelete.setOnClickListener {
            currentDownloadedFile?.let { file ->
                if (file.exists() && file.delete()) {
                    tvStatus.text = "Status: File deleted"
                    btnView.visibility = View.GONE
                    btnDelete.visibility = View.GONE
                    currentDownloadedFile = null
                    Toast.makeText(this, "File successfully deleted", Toast.LENGTH_SHORT).show()
                }
            }
        }
    }

    private fun showTutorialPopup() {
        val inflater = getSystemService(LAYOUT_INFLATER_SERVICE) as LayoutInflater
        val popupView = inflater.inflate(R.layout.popup_tutorial, null)

        val popupWindow = PopupWindow(
            popupView,
            androidx.constraintlayout.widget.ConstraintLayout.LayoutParams.MATCH_PARENT,
            androidx.constraintlayout.widget.ConstraintLayout.LayoutParams.MATCH_PARENT,
            true
        )

        val cbDoNotShowAgain = popupView.findViewById<CheckBox>(R.id.cbDoNotShowAgain)
        val btnOk = popupView.findViewById<Button>(R.id.btnOkPopup)

        btnOk.setOnClickListener {
            if (cbDoNotShowAgain.isChecked) {
                val sharedPrefs = getSharedPreferences("AppPreferences", Context.MODE_PRIVATE)
                sharedPrefs.edit().putBoolean("show_tutorial", false).apply()
            }
            popupWindow.dismiss()
        }

        val mainLayout = findViewById<View>(android.R.id.content)
        popupWindow.showAtLocation(mainLayout, Gravity.CENTER, 0, 0)
    }

    private fun downloadPdfFile(fileUrl: String, storageDir: File) {
        tvStatus.text = "Status: Downloading..."
        btnView.visibility = View.GONE
        btnDelete.visibility = View.GONE

        lifecycleScope.launch(Dispatchers.IO) {
            val fileName = "journal_${System.currentTimeMillis()}.pdf"
            val targetFile = File(storageDir, fileName)

            try {
                val url = URL(fileUrl)
                val connection = url.openConnection() as HttpURLConnection
                connection.requestMethod = "GET"
                connection.connect()

                val contentType = connection.contentType
                if (contentType != null && contentType.contains("application/pdf")) {

                    val inputStream = connection.inputStream
                    val outputStream = FileOutputStream(targetFile)

                    val buffer = ByteArray(4096)
                    var bytesRead: Int

                    while (inputStream.read(buffer).also { bytesRead = it } != -1) {
                        outputStream.write(buffer, 0, bytesRead)
                    }

                    outputStream.close()
                    inputStream.close()

                    withContext(Dispatchers.Main) {
                        tvStatus.text = "Status: Download complete"
                        currentDownloadedFile = targetFile
                        btnView.visibility = View.VISIBLE
                        btnDelete.visibility = View.VISIBLE
                    }
                } else {
                    withContext(Dispatchers.Main) {
                        tvStatus.text = "Status: File not found or not a PDF"
                    }
                }
                connection.disconnect()
            } catch (e: Exception) {
                e.printStackTrace()
                withContext(Dispatchers.Main) {
                    tvStatus.text = "Status: Download failed (${e.message})"
                }
            }
        }
    }

    private fun openPdfFile(file: File) {
        try {
            val uri = FileProvider.getUriForFile(this@MainActivity, "${packageName}.provider", file)
            val intent = Intent(Intent.ACTION_VIEW)
            intent.setDataAndType(uri, "application/pdf")
            intent.flags = Intent.FLAG_GRANT_READ_URI_PERMISSION
            startActivity(intent)
        } catch (e: ActivityNotFoundException) {
            Toast.makeText(
                this@MainActivity,
                "No application found to open PDF files",
                Toast.LENGTH_LONG
            ).show()
        } catch (e: Exception) {
            Toast.makeText(this@MainActivity, "Error opening file", Toast.LENGTH_SHORT).show()
        }
    }
}