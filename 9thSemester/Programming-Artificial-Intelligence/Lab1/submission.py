import collections
import math

############################################################
# Problem 1a

def findAlphabeticallyLastWord(text):
    """
    Given a string |text|, return the word in |text| that comes last
    alphabetically (that is, the word that would appear last in a dictionary).
    A word is defined by a maximal sequence of characters without whitespaces.
    You might find max() and list comprehensions handy here.
    """
    # BEGIN_YOUR_CODE (our solution is 1 line of code, but don't worry if you deviate from this)
    last = ""
    words = [item.lower().replace(',', '').replace('.', '') for item in text.split()]
    for word in words:
        if last < word:
            last = word
    return last
    # END_YOUR_CODE

############################################################
# Problem 1b

def euclideanDistance(loc1, loc2):
    """
    Return the Euclidean distance between two locations, where the locations
    are pairs of numbers (e.g., (3, 5)).
    """
    # BEGIN_YOUR_CODE (our solution is 1 line of code, but don't worry if you deviate from this)
    return math.sqrt((loc1[0] - loc2[0]) ** 2 + (loc1[1] - loc2[1]) ** 2)
    # END_YOUR_CODE

############################################################
# Problem 1c

def mutateSentences(sentence):
    """
    Given a sentence (sequence of words), return a list of all "similar"
    sentences.
    We define a sentence to be similar to the original sentence if
      - it as the same number of words, and
      - each pair of adjacent words in the new sentence also occurs in the original sentence
        (the words within each pair should appear in the same order in the output sentence
         as they did in the orignal sentence.)
    Notes:
      - The order of the sentences you output doesn't matter.
      - You must not output duplicates.
      - Your generated sentence can use a word in the original sentence more than
        once.
    Example:
      - Input: 'the cat and the mouse'
      - Output: ['and the cat and the', 'the cat and the mouse', 'the cat and the cat', 'cat and the cat and']
                (reordered versions of this list are allowed)
    """
    # BEGIN_YOUR_CODE (our solution is 20 lines of code, but don't worry if you deviate from this)
    words = sentence.split()
    n = len(words)
    if n == 0:
        return []
    if n == 1:
        return [sentence]

    # Adjacency: Word -> list of possible next words (preserve occurrences/order)
    adj = collections.defaultdict(list)
    for a, b in zip(words, words[1:]):
        adj[a].append(b)

    results = set()

    def dfs(path):
        if len(path) == n:
            results.add(" ".join(path))
            return
        last = path[-1]
        for nxt in adj.get(last, ()):
            path.append(nxt)
            dfs(path)
            path.pop()

    # Try every distinct word as a starting node (some starts may not reach full length)
    for start in set(words):
        dfs([start])

    return list(results)
    # END_YOUR_CODE

############################################################
# Problem 1d

def sparseVectorDotProduct(v1, v2):
    """
    Given two sparse vectors |v1| and |v2|, each represented as collections.defaultdict(float), return
    their dot product.
    You might find it useful to use sum() and a list comprehension.
    This function will be useful later for linear classifiers.
    """
    # BEGIN_YOUR_CODE (our solution is 4 lines of code, but don't worry if you deviate from this)
    # Efficiency trick :)
    if len(v1) > len(v2):
        v1, v2 = v2, v1
    return sum(value * v2.get(key, 0.0) for key, value in v1.items())
    # END_YOUR_CODE

############################################################
# Problem 1e

def incrementSparseVector(v1, scale, v2):
    """
    Given two sparse vectors |v1| and |v2|, perform v1 += scale * v2.
    This function will be useful later for linear classifiers.
    """
    # BEGIN_YOUR_CODE (our solution is 2 lines of code, but don't worry if you deviate from this)
    for k, val in v2.items():
        v1[k] = v1.get(k, 0.0) + scale * val
    return v1
    # END_YOUR_CODE

############################################################
# Problem 1f

def findSingletonWords(text):
    """
    Splits the string |text| by whitespace and returns the set of words that
    occur exactly once.
    You might find it useful to use collections.defaultdict(int).
    """
    # BEGIN_YOUR_CODE (our solution is 4 lines of code, but don't worry if you deviate from this)
    singleton_words = collections.defaultdict(int)
    for word in text.split():
        singleton_words[word] += 1
    return { word for word, count in singleton_words.items() if count == 1}
    # END_YOUR_CODE

############################################################
# Problem 1g

def computeLongestPalindromeLength(text):
    """
    A palindrome is a string that is equal to its reverse (e.g., 'ana').
    Compute the length of the longest palindrome that can be obtained by deleting
    letters from |text|.
    For example: the longest palindrome in 'animal' is 'ama'.
    Your algorithm should run in O(len(text)^2) time.
    You should first define a recurrence before you start coding.
    """
    # BEGIN_YOUR_CODE (our solution is 19 lines of code, but don't worry if you deviate from this)
    n = len(text)
    if n == 0:
        return 0

    dp = [[0] * n for _ in range(n)]

    # Substrings of length 1 are palindromes of length 1
    for i in range(n):
        dp[i][i] = 1

    # Build for increasing substring lengths
    for length in range(2, n + 1):
        for i in range(n - length + 1):
            j = i + length - 1
            if text[i] == text[j]:
                if length == 2:
                    dp[i][j] = 2
                else:
                    dp[i][j] = 2 + dp[i + 1][j - 1]
            else:
                dp[i][j] = max(dp[i + 1][j], dp[i][j - 1])

    return dp[0][n - 1]
    # END_YOUR_CODE
