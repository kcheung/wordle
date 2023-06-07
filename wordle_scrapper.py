import re
import requests
from bs4 import BeautifulSoup

URL = 'https://www.wordunscrambler.net/word-list/wordle-word-list'

webpage = requests.get(URL)
contents = BeautifulSoup(webpage.content, "html.parser")

words = contents.find_all('a', {
    'href': re.compile(r'/unscramble/')
  })

with open("words.txt", "w") as file:
  for anchor in words:
    file.write(anchor.string + "\n")