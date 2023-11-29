package Lab4.Task4.Entities;

import java.util.LinkedList;
import java.util.List;
import java.util.UUID;

public class Book {
    private final UUID id;
    private String author;
    private String title;
    private List<String> genres;

    public Book(String author, String title, List<String> genres) {
        id = UUID.randomUUID();

        setAuthor(author);
        setTitle(title);
        setGenres(genres);
    }

    public UUID getId() {
        return id;
    }

    public String getAuthor() {
        return author;
    }

    public void setAuthor(String author) {
        if (author.trim().isEmpty()) throw new IllegalArgumentException("Author must be non-empty");
        this.author = author;
    }

    public String getTitle() {
        return title;
    }

    public void setTitle(String title) {
        if (title.trim().isEmpty()) throw new IllegalArgumentException("Title must be non-empty");
        this.title = title;
    }

    public List<String> getGenres() {
        return genres;
    }

    public void setGenres(List<String> genres) {
        if (genres != null) {
            this.genres = genres;
        } else {
            this.genres = new LinkedList<>();
        }
    }

    public void addGenre(String genre) {
        if (genres.contains(genre)) throw new IllegalArgumentException("There's already genre like this");
        if (genre.trim().isEmpty()) throw new IllegalArgumentException("Genre must be non-empty");

        genres.add(genre);
    }

    public void deleteGenre(String genre) {
        if (!genres.contains(genre)) throw new IllegalArgumentException("There's no genre like this");
        if (genre.trim().isEmpty()) throw new IllegalArgumentException("Genre must be non-empty");

        genres.remove(genre);
    }

    @Override
    public String toString() {
        var sb = new StringBuilder();

        sb.append(String.format("ID: %s%n", this.id));
        sb.append(String.format("Title: \"%s\"%n", this.title));
        sb.append(String.format("Author: %s%n", this.author));

        if (genres.isEmpty()) {
            sb.append("Genres: None");
            return sb.toString();
        }

        sb.append(String.format("Genres: %n"));
        for (int i = 0; i < this.genres.size(); i++) {
            var genre = this.genres.get(i);
            sb.append(String.format("\t%d. %s%n", i+1, genre));
        }

        return sb.toString();
    }
}
