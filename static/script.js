const button = document.querySelector("button");
const input = document.querySelector("input");
const newsContainer = document.getElementById("#2");

function displayNews(news) {
    newsContainer.innerHTML = ''; // Очищаем контейнер перед добавлением новых новостей

    if (news.length === 0) {
        newsContainer.innerHTML = '<p>No news found</p>';
        return;
    }

    news.forEach(article => {
        const articleElement = document.createElement('div');
        articleElement.classList.add('news-article');

        const title = document.createElement('h3');
        title.textContent = article.title;

        const link = document.createElement('a');
        link.href = article.url;
        link.textContent = 'Read more';
        link.target = "_blank";

        const source = document.createElement('p');
        source.textContent = `Source: ${article.source}`;

        const date = document.createElement('p');
        date.textContent = `Published on: ${article.date}`;

        articleElement.appendChild(title);
        articleElement.appendChild(link);
        articleElement.appendChild(source);
        articleElement.appendChild(date);

        newsContainer.appendChild(articleElement);
    });
}

async function fetchNews() {
    const cryptoName = input.value.trim();

    if (!cryptoName) {
        alert('Please enter a cryptocurrency name');
        return;
    }

    try {
        const response = await fetch(`/news?query=${cryptoName}`);
        const data = await response.json();
        displayNews(data);
    } catch (error) {
        console.error('Error fetching news:', error);
        newsContainer.innerHTML = '<p>Error fetching news. Please try again later.</p>';
    }
}

button.addEventListener('click', fetchNews);

input.addEventListener('keypress', (event) => {
    if (event.key === 'Enter') {
        fetchNews();
    }
});
