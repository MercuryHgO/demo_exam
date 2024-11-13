CREATE TABLE partners (
    partner_type text NOT NULL,
    partner_name text NOT NULL,
    director text NOT NULL,
    email text NOT NULL,
    phone text NOT NULL,
    legal_address text NOT NULL,
    inn text NOT NULL,
    rating smallint NOT NULL,
    id text PRIMARY KEY NOT NULL
);


CREATE TABLE product_types (
    product_type text PRIMARY KEY NOT NULL,
    coefficient real NOT NULL
);

CREATE TABLE products (
    product_type text NOT NULL,
    product_name text NOT NULL,
    article_number text NOT NULL,
    minimum_cost integer NOT NULL,
    id text PRIMARY KEY NOT NULL,
    CONSTRAINT fk_product_type
    FOREIGN KEY (product_type) REFERENCES product_types(product_type) ON DELETE CASCADE
);

CREATE TABLE sales (
    product_id text NOT NULL,
    quantity integer NOT NULL,
    sale_date date NOT NULL,
    partner_id text NOT NULL,
    id text PRIMARY KEY NOT NULL,
    CONSTRAINT fk_partner
    FOREIGN KEY (partner_id) REFERENCES partners(id) ON DELETE CASCADE,
    CONSTRAINT fk_product
    FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE
);

INSERT INTO partners VALUES ('ЗАО', 'База Строитель', 'Иванова Александра Ивановна', 'aleksandraivanova@ml.ru', '493 123 45 67', '652050, Кемеровская область, город Юрга, ул. Лесная, 15', '2222455179', 7, 'b206fb4f-6003-4dbc-b280-8fdefcb6160f');
INSERT INTO partners VALUES ('ООО', 'Паркет 29', 'Петров Василий Петрович', 'vppetrov@vl.ru', '987 123 56 78', '164500, Архангельская область, город Северодвинск, ул. Строителей, 18', '3333888520', 7, '0d1cfc05-56a7-4bc9-8d67-a28026519d51');
INSERT INTO partners VALUES ('ПАО', 'Стройсервис', 'Соловьев Андрей Николаевич', 'ansolovev@st.ru', '812 223 32 00', '188910, Ленинградская область, город Приморск, ул. Парковая, 21', '4440391035', 7, '4d07febd-4f7e-40a3-b29f-2670c12f155e');
INSERT INTO partners VALUES ('ОАО', 'Ремонт и отделка', 'Воробьева Екатерина Валерьевна', 'ekaterina.vorobeva@ml.ru', '444 222 33 11', '143960, Московская область, город Реутов, ул. Свободы, 51', '1111520857', 5, '3e3cf9b9-0835-426c-ba69-c72e608d89a2');
INSERT INTO partners VALUES ('ЗАО', 'МонтажПро', 'Степанов Степан Сергеевич', 'stepanov@stepan.ru', '912 888 33 33', '309500, Белгородская область, город Старый Оскол, ул. Рабочая, 122', '5552431140', 10, '0f4ab536-9b03-4b28-a603-1e73f0cd3b01');

INSERT INTO product_types VALUES ('Ламинат', 2.35);
INSERT INTO product_types VALUES ('Массивная доска', 5.15);
INSERT INTO product_types VALUES ('Паркетная доска', 4.34);
INSERT INTO product_types VALUES ('Пробковое покрытие', 1.5);


INSERT INTO products VALUES ('Паркетная доска', 'Паркетная доска Ясень темный однополосная 14 мм', '8758385', 4456.90, '1ad4c682-f147-4ac9-a8e5-47bd07847315');
INSERT INTO products VALUES ('Паркетная доска', 'Инженерная доска Дуб Французская елка однополосная 12 мм', '8858958', 7330.99, '85a8eb9e-516f-42ff-bb78-bc700f0a24d6');
INSERT INTO products VALUES ('Ламинат', 'Ламинат Дуб дымчато-белый 33 класс 12 мм', '7750282', 1799.33, '61e94394-ab6a-4f27-9504-f7cbf002acac');
INSERT INTO products VALUES ('Ламинат', 'Ламинат Дуб серый 32 класс 8 мм с фаской', '7028748', 3890.41, '0b1ebe6e-e63c-4ac4-903b-63e117ada6a6');
INSERT INTO products VALUES ('Пробковое покрытие', 'Пробковое напольное клеевое покрытие 32 класс 4 мм', '5012543', 5450.59, '8371df83-39ba-4a39-be05-547cc6b97158');
