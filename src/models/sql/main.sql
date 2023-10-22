CREATE TABLE feedbacks (
    id       SERIAL PRIMARY KEY,
    username VARCHAR(100) NOT NULL,
    email    VARCHAR(200) NOT NULL,
    message  VARCHAR(1000) NOT NULL
);

/*
Как айти компании корректировать все цены своих услуг, товаров и опций, исходя из меняющейся картины экономики в странах.
учтем: 
1. курс валют на мировом рынке (будем парсить и сохранять в local storage сервера)
2. корректировку исходя из инфляции: ratio - умножитель
3. корректировку исходя из прочих причин: adder - плюс/минус

В итоге эти корректировщики сохранятся в local storage сервера и будут доступны во всех вьюхах
для рассчета цены. При создании объекта работаем с рублями. Если пользователь в Европе, работаем с евро.
В других случаях пока остается доллар. В будущем нужно и другие добавить.

Получаем последний объект price_corrects, оттуда считываем и сохраняем в local storage нужную
валюту currency, ratio и adder.

Цена такая для рубля: object.price * ratio + adder.
Например, стоит 10_000 рублей, а ratio 1.1, adder 250, получаем 10_000 * 1.1 + 250 = 11250
Для евро: (10_000 * 0,00985943 (курс к рублю) * 1.1) + (250 * 0,00985943) = 101,0543

Также записи в таблице price_corrects имеют время создания изменения для отражения и этого момента пользователям. 
Иможно так даже выводить список изменений цен по времени.
*/
CREATE TABLE price_corrects (
    id       SERIAL PRIMARY KEY,
    currency VARCHAR(10) NOT NULL,  -- валюта корректировки
    ratio    FLOAT NOT NULL,        -- коэффициент валюты (множитель)
    adder    INT NOT NULL,          -- сумматор валюты
    created  TIMESTAMP NOT NULL     -- время создания 
); 


CREATE TABLE orders (
    id             SERIAL PRIMARY KEY,
    title          VARCHAR(100) NOT NULL,
    title_en       VARCHAR(100) NOT NULL,
    types          SMALLINT NOT NULL, -- 1 услуга, 2 товар, 3 работа
    object_id      INT NOT NULL,
    username       VARCHAR(200) NOT NULL,
    email          VARCHAR(200) NOT NULL,
    description    VARCHAR(1000),
    description_en VARCHAR(1000),
    created        TIMESTAMP NOT NULL,
    user_id        INT NOT NULL,
    price          INT NOT NULL,
    price_acc      INT
);

CREATE TABLE order_files (
    id       SERIAL PRIMARY KEY,
    order_id INT NOT NULL,
    src      VARCHAR(500) NOT NULL,

    CONSTRAINT fk_order_files
        FOREIGN KEY(order_id)
            REFERENCES orders(id)
);
CREATE INDEX order_files_id_idx ON order_files (order_id);

CREATE TABLE users (
    id       SERIAL PRIMARY KEY,
    username VARCHAR(100) NOT NULL,
    email    VARCHAR(100) NOT NULL,
    password VARCHAR(1000) NOT NULL,
    bio      VARCHAR(500),
    image    VARCHAR(500),
    perm     SMALLINT NOT NULL,

    UNIQUE(username),
    UNIQUE(email)
);

CREATE TABLE chats (
    id                SERIAL PRIMARY KEY,
    user_id           INT NOT NULL,
    created           TIMESTAMP NOT NULL,

    CONSTRAINT fk_chat_creator
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);
CREATE INDEX chats_user_id_idx ON chats (user_id);

CREATE TABLE messages (
    id      SERIAL PRIMARY KEY,     -- id объекта
    user_id INT NOT NULL,           -- id создателя
    chat_id INT NOT NULL,           -- id чата
    created TIMESTAMP NOT NULL,     -- когда создано
    content VARCHAR(5000),          -- текст
    view    SMALLINT NOT NULL,      -- создано / показано / прочитано
    types   SMALLINT NOT NULL,      -- обычное / изменено / удалено

    CONSTRAINT fk_message_creator        -- связь с создателем
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);
CREATE INDEX messages_user_id_idx ON messages (user_id);

/*
RUB рубль
USD доллар США
EUR евро
GBP Фунт стерлингов
BYN Белорусский рубль
GEL Грузинский лари
JPY Японская йена
CHF Швейцарский франк
TRY Турецкая лира
PLN Польский злотый
CNY Китайский юань
CAD Канадский доллар
KZT Казахстанский тенге
INR Индийская рупия
*/ 
CREATE TABLE cookie_users (
    id         SERIAL PRIMARY KEY,
    ip         VARCHAR(100) NOT NULL, -- ip адрес пользователя
    device     SMALLINT NOT NULL,     -- комп - смартфон - планшет
    linguage   SMALLINT NOT NULL,     -- язык
    template   SMALLINT NOT NULL,     -- шаблон
    currency   VARCHAR(10),           -- валюта RUB, USD, EUR и тд
    city_ru    VARCHAR(150),          -- город по русски
    city_en    VARCHAR(150),          -- город по английски
    region_ru  VARCHAR(150),          -- регион по русски
    region_en  VARCHAR(150),          -- регион по английски
    country_ru VARCHAR(150),          -- страна по русски
    country_en VARCHAR(150),          -- страна по английски
    height     FLOAT NOT NULL,
    seconds    INT NOT NULL,
    created    TIMESTAMP NOT NULL     -- когда создан пользователь
);
CREATE TABLE cookie_stats (
    id       SERIAL PRIMARY KEY,
    user_id  INT NOT NULL,          -- связь с пользователем куки
    page     SMALLINT NOT NULL,     -- номер страницы, которая просматривается
    link     VARCHAR(200) NOT NULL, -- ссылка страницы
    title    VARCHAR(200) NOT NULL, -- название страницы
    height   FLOAT NOT NULL,        -- высота просмотра страницы
    seconds  INT NOT NULL,          -- секунды нахождения страницы
    created  TIMESTAMP NOT NULL,    -- когда создана запись
    template VARCHAR(100) NOT NULL DEFAULT 'rhythm', -- вид шаблона

    CONSTRAINT fk_cookie_stat_user
        FOREIGN KEY(user_id)
            REFERENCES cookie_users(id)
);

CREATE TABLE tags (
    id        SERIAL PRIMARY KEY,
    name      VARCHAR(100) NOT NULL,
    name_en   VARCHAR(100) NOT NULL,
    position  SMALLINT NOT NULL,
    count     SMALLINT NOT NULL,
    user_id   INT NOT NULL,
    view      INT NOT NULL,
    height    FLOAT NOT NULL,
    seconds   INT NOT NULL,

    CONSTRAINT fk_tag_creator
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);

CREATE TABLE tags_items (
    id      SERIAL PRIMARY KEY,
    tag_id  INT NOT NULL,
    item_id INT NOT NULL,
    types   SMALLINT NOT NULL, -- блог, услуга, товар ......
    created TIMESTAMP NOT NULL
);

CREATE TABLE categories ( 
    id             SERIAL PRIMARY KEY,
    name           VARCHAR(100) NOT NULL,
    name_en        VARCHAR(100) NOT NULL,
    description    VARCHAR(500),
    description_en VARCHAR(500),
    position       SMALLINT NOT NULL,
    image          VARCHAR(500),
    count          SMALLINT NOT NULL,
    view           INT NOT NULL,
    height         FLOAT NOT NULL,
    seconds        INT NOT NULL,
    types          SMALLINT NOT NULL, -- категория блога, категория услуги ......
    slug           VARCHAR(100) NOT NULL,

    UNIQUE(slug)
);
INSERT INTO categories (name,name_en,description,description_en,position,image,count,view,height,seconds,types,slug)
VALUES ('Как заказать проект','How to order a project','','',1,'',0,0,0.0,0,6,'how_to_order_a_project') ON CONFLICT DO NOTHING;
INSERT INTO categories (name,name_en,description,description_en,position,image,count,view,height,seconds,types,slug)
VALUES ('Что мы о Вас знаем','What do we know abou you','','',2,'',0,0,0.0,0,6,'what_do_we_know_abou_you') ON CONFLICT DO NOTHING;
INSERT INTO categories (name,name_en,description,description_en,position,image,count,view,height,seconds,types,slug)
VALUES ('Этапы создания проектов','Stages of project creation','','',3,'',0,0,0.0,0,6,'stages_of_project_creation') ON CONFLICT DO NOTHING;
INSERT INTO categories (name,name_en,description,description_en,position,image,count,view,height,seconds,types,slug)
VALUES ('Все услуги','All services','','',1,'',0,0,0.0,0,2,'all-services') ON CONFLICT DO NOTHING;
INSERT INTO categories (name,name_en,description,description_en,position,image,count,view,height,seconds,types,slug)
VALUES ('Веб-разработка','Service of web','','',2,'',0,0,0.0,0,2,'service-web') ON CONFLICT DO NOTHING;
INSERT INTO categories (name,name_en,description,description_en,position,image,count,view,height,seconds,types,slug)
VALUES ('Все работы','All works','','',1,'',0,0,0.0,0,5,'all-works') ON CONFLICT DO NOTHING;
INSERT INTO categories (name,name_en,description,description_en,position,image,count,view,height,seconds,types,slug)
VALUES ('Веб-разработка','Web development','','',2,'',0,0,0.0,0,5,'web-development') ON CONFLICT DO NOTHING;
INSERT INTO categories (name,name_en,description,description_en,position,image,count,view,height,seconds,types,slug)
VALUES ('Все статьи','All articles','','',1,'',0,0,0.0,0,1,'all-articles') ON CONFLICT DO NOTHING;
INSERT INTO categories (name,name_en,description,description_en,position,image,count,view,height,seconds,types,slug)
VALUES ('Все статьи','All wikis','','',1,'',0,0,0.0,0,4,'all-wikis') ON CONFLICT DO NOTHING;
INSERT INTO categories (name,name_en,description,description_en,position,image,count,view,height,seconds,types,slug)
VALUES ('Все товары','All stores','','',1,'',0,0,0.0,0,3,'all-stores') ON CONFLICT DO NOTHING;


CREATE TABLE items (
    id             SERIAL PRIMARY KEY,
    title          VARCHAR(100) NOT NULL,
    title_en       VARCHAR(100) NOT NULL,
    description    VARCHAR(500),
    description_en VARCHAR(500), 
    content        VARCHAR(30000),
    content_en     VARCHAR(30000),
    link           VARCHAR(500),
    image          VARCHAR(500),
    is_active      boolean NOT NULL,
    price          INT NOT NULL,
    user_id        INT NOT NULL,
    created        TIMESTAMP NOT NULL,
    position       SMALLINT NOT NULL,
    view           INT NOT NULL,
    height         FLOAT NOT NULL,
    seconds        INT NOT NULL,
    price_acc      INT,
    types          SMALLINT NOT NULL, -- блог, услуга, товар ......
    slug           VARCHAR(100) NOT NULL,

    UNIQUE(slug),

    CONSTRAINT fk_store_creator
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);
CREATE INDEX items_creator_idx ON items (user_id);


CREATE TABLE item_comments (
    id        SERIAL PRIMARY KEY,
    comment   VARCHAR(1000) NOT NULL,
    item_id   INT NOT NULL,
    user_id   INT NOT NULL,
    parent_id INT,
    created   TIMESTAMP NOT NULL,

    CONSTRAINT fk_item_comment
        FOREIGN KEY(item_id)
            REFERENCES items(id),

    CONSTRAINT fk_user_item_comment
        FOREIGN KEY(user_id)
            REFERENCES users(id),

    CONSTRAINT fk_item_parent_comment
        FOREIGN KEY(parent_id)
            REFERENCES item_comments(id)
);
CREATE INDEX item_comments_id_idx ON item_comments (item_id);
CREATE INDEX item_comments_user_id_idx ON item_comments (user_id);

CREATE TABLE category ( 
    id          SERIAL PRIMARY KEY,
    category_id INT NOT NULL,
    item_id     INT NOT NULL,
    types       SMALLINT NOT NULL, -- блог, услуга, товар ......

   CONSTRAINT fk_category_cat
        FOREIGN KEY(category_id)
            REFERENCES categories(id),

   CONSTRAINT fk_category_item
        FOREIGN KEY(item_id)
            REFERENCES items(id)
);

CREATE TABLE files (
    id             SERIAL PRIMARY KEY,
    user_id        INT NOT NULL,
    item_id        INT NOT NULL,
    item_types     SMALLINT NOT NULL, -- блог, услуга, товар ......
    types          SMALLINT NOT NULL, -- фото, видео, документ  ......
    src            VARCHAR(500) NOT NULL,
    description    VARCHAR(500),
    description_en VARCHAR(500),
    position       SMALLINT NOT NULL,
    view           INT NOT NULL,
    seconds        INT NOT NULL,

    UNIQUE(src)
);

-- это веб-сервисы (например, большой магазин или моб приложение ресторана)
CREATE TABLE web_services (
    id             SERIAL PRIMARY KEY,
    name           VARCHAR(100) NOT NULL,
    name_en        VARCHAR(100) NOT NULL,
    description    VARCHAR(10000),
    description_en VARCHAR(10000),
    position       SMALLINT NOT NULL,
    count          SMALLINT NOT NULL,
    level          SMALLINT NOT NULL,
    user_id        INT NOT NULL,
    view           INT NOT NULL,
    height         FLOAT NOT NULL,
    seconds        INT NOT NULL
);

-- это категория опции (например, rust, python, react native)
CREATE TABLE serve_categories ( 
    id             SERIAL PRIMARY KEY,
    name           VARCHAR(100) NOT NULL,
    name_en        VARCHAR(100) NOT NULL,
    description    VARCHAR(10000),
    description_en VARCHAR(10000),
    category_id    INT NOT NULL, 
    position       SMALLINT NOT NULL,
    count          SMALLINT NOT NULL,
    default_price  INT NOT NULL, -- сумма всех опуий по умолчанию.
    user_id        INT NOT NULL,
    view           INT NOT NULL,
    height         FLOAT NOT NULL,
    seconds        INT NOT NULL,

    CONSTRAINT fk_web_services
        FOREIGN KEY(category_id)
            REFERENCES web_services(id)
);

-- это опции (например, продвинутая админка)
CREATE TABLE serve ( 
    id             SERIAL PRIMARY KEY,
    name           VARCHAR(100) NOT NULL,
    name_en        VARCHAR(100) NOT NULL,
    description    VARCHAR(10000),
    description_en VARCHAR(10000),
    position       SMALLINT NOT NULL,
    category_id    INT NOT NULL,
    price          INT NOT NULL,
    man_hours      SMALLINT NOT NULL, 
    is_default     BOOLEAN NOT NULL, -- опция по умолчанию, т.е. без которой работа невозможна (например, админка)
    user_id        INT NOT NULL,
    web_service_id INT NOT NULL,
    height         FLOAT NOT NULL,
    seconds        INT NOT NULL,
    serve_id       INT,
    view           INT NOT NULL,

    CONSTRAINT fk_serve_category
        FOREIGN KEY(category_id)
            REFERENCES serve_categories(id),
    CONSTRAINT fk_serve_creator
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);

-- связь опции с объетками сервисов, работ, товаров
CREATE TABLE serve_items (
    id       SERIAL PRIMARY KEY,
    serve_id INT NOT NULL,
    item_id  INT NOT NULL,
    types    SMALLINT NOT NULL
);

-- это те web_services, которые привязываются к объеткам.
-- бывают открытые (активные) и дополнительные.
CREATE TABLE web_services_items (
    id          SERIAL PRIMARY KEY,
    category_id INT NOT NULL,     -- тех. категория (например, создание среднего магазина)
    item_id     INT NOT NULL,
    types       SMALLINT NOT NULL, -- блог, товар ......
    is_active   SMALLINT NOT NULL -- тип: 1 - активно, 2 - неактивно
);

CREATE TABLE stat_pages (
    id      SERIAL PRIMARY KEY,
    types   SMALLINT NOT NULL,  -- главная страница, инфо ......
    view    INT NOT NULL,
    height  FLOAT NOT NULL,
    seconds INT NOT NULL
);
