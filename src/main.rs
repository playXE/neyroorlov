const MAIN_PHARSES: &'static [&'static str] = &[
    "БАРНАУЛ ОБЩИЙ СБОР",
    "Бип-боп",
    "Вахтер красава",
    "Вы победили",
    "Геюга",
    "ГОСПАДИ ПАМАГИ",
    "Давай дружить. Разбань",
    "Давай я тебя забаню, а ты заплачешь???",
    "Дай админку",
    "Дмитрий Орлов — лучший пользователь ТЖ?",
    "Древнее зло пробудилось",
    "ЕДУ В КАЗАХСТАН",
    "За что меня забанил вахтер???",
    "Закончил школу уже?",
    "Зачем либералы клевещут на сталина??",
    "Зачем польский бариста Дитковский добавил меня в чс??",
    "Зачем русские рвутся, когда их просят говорить Беларусь?",
    "Зашейся",
    "Знаешь кто твоя мать?",
    "Как дела?",
    "Какая позиция медузы по Крыму????",
    "Когда в Беларуси протесты?",
    "Когда русские перестанут терпеть?",
    "Кринж",
    "КТО ХУЖЕ: СТАЛИН ИЛИ ГИТЛЕР",
    "Ладно",
    "Лучше не стало",
    "Меня забанили!",
    "Мечтают ли нейроны о нейроовцах?",
    "Нашел твою маму. Что с ней делать???",
    "Не пизди",
    "Не пизди пожалуйста умоляю",
    "Не умничай",
    "Нихуя ты умный братан",
    "Ой бляяяяя",
    "Помогите мне пожалуйста!",
    "Почему злые олигархи не дади сталину построить в СССР демократию??????",
    "Почему Лукашенко терпит этих белорусов???",
    "Почему на дтф сидят так много обоссанных коммуняк?",
    "Правый хуже пидораса",
    "Привет",
    "Привет БРАТ",
    "Приезжай в Казахстан угощу кумысом",
    "Пришел сюда за этим",
    "РАЗБАНЬ",
    "Рот закрой",
    "Сколько граждане России будут терпеть произвол Вахтера???",
    "Слабак",
    "Сразу видно, украинские корни",
    "Твоя мать мать",
    "Ты бы прошел тест на руssкого?",
    "Ты привился?",
    "Ты смотрел боку но пико?",
    "Ты узбек? Давай дружить. Нужно в Ташкент мне",
    "Ты уважаешь Навального???",
    "Умный дохуя?",
    "Хочу чтобы Казахстан стал 51 штатом США",
    "Чечня превыше всего",
    "Что не так с русскими???",
    "Я думаю ты запизделся",
    "Я казахский патриот",
];

const COMMON_TEXT: &'static [&'static str] = &[
    "ЗАБЫТЬ СПРОСИЛИ АХАХАХАХАХА!!!!",
    "@Шахтёр ПРЕСЛЕДОВАНИЯ УПОМИНАНИЯМИ!!!!",
    "JAL",
    "NE RVIS",
    "Ne rviS i hvatit menya travIt",
    "Ne rvis. Я серьезно",
    "А правда, что российское государство специально создаёт образ диких агрессивных русских, чтобы оправдать свою власть????",
    "А Я БОЯЛСЯ ЧТО Я ОДИН ТАКОЙ",
    "А?",
    "Антисоветчик всегда гомофоб",
    "БАРНАУЛ ОБЩИЙ СБОР",
    "Барнаул уже выступил против вопиющего феминизма?????",
    "Бггг",
    "Беларусь считается зарубежом для России????",
    "Белорусы могут объяснить, почему они не выходят на митинги?",
    "Бип-боп",
    "Больше никаких вбросов",
    "Брат, тебе тут круглосуточно разъясняют, почему ты пишешь хуйню",
    "В чем я не прав??",
    "Вахтер красава",
    "Вахтер, может хватит рваться??? Разбань Орлова",
    "Ведёшь себя как порохбот",
    "Все так",
    "Всё равно хуйню сказал",
    "ВСЁ ЯСНО",
    "Вы победили",
    "ВЫДВИГАЮ НАРОДНЫЙ УЛЬТИМАТУМ ПРОТИВ ЦЫПЛУХИНА И ВАХТЕРА",
    "Высеры шизофреников, которым лишь бы никак все быть",
    "Геюга",
    "ГЛАВНОЕ НЕ БУХТЕТЬ???",
    "Говоришь как Навальный",
    "ГОСПАДИ ПАМАГИ",
    "Да",
    "Да как этот птушник постоянно у меня в чс оказывается",
    "Да. В чем я не прав?",
    "Давай БЕЗ ЭТОГО",
    "Давай дружить. Разбань",
    "Давай я тебя забаню, а ты заплачешь???",
    "Дай админку",
    "Двойные стандарты",
    "Двойные стандарты, жаль",
    "Дмитрий Орлов — лучший пользователь ТЖ?",
    "Добавь в чат",
    "ДОГОВОРИЛСЯ С ВАХТЕРОМ О МОЕМ РАЗБАНЕ",
    "Долбаебы рвутся с рыночных отношений????",
    "Древнее зло пробудилось",
    "ЕДУ В КАЗАХСТАН",
    "Если ты так думаешь, то ЖАЛКО ТЕБЯ",
    "Жалко ущемленных славян",
    "Жаль",
    "Жаль бедной Украине это не помогает",
    "За что меня забанил вахтер???",
    "За что??????",
    "Задоначу тж, чтобы его признали иноагентом",
    "Закончил школу уже?",
    "Зачем влад Цып нанимал двух вахтеров, если по выходным всё равно никто не работает?????????",
    "Зачем либералы клевещут на сталина??",
    "Зачем польский бариста Дитковский добавил меня в чс??",
    "Зачем порвался?",
    "Зачем порвался???",
    "Зачем рваться с кремлеботов??",
    "Зачем русские рвутся, когда их просят говорить Беларусь?",
    "Зачем советов двигает кремлёвскую повестку про развал запада?",
    "Зачем тебя распидорасило тогда так, что ты продолжаешь рваться и писать мне??",
    "Зачем ты насрал в треде??? Убирай",
    "Зачем ты порвался?",
    "Зачем УКРАИНЦЫ позволили отобрать у себя крым?",
    "Зачем хаким продолжает угрожать каждому встречному в интернете???",
    "Зачем я занимаюсь всей этой хуйней? Помогите",
    "Зашейся",
    "Звучит справедливо",
    "Знаешь кто твоя мать?",
    "И ведь лайкают даже такие высеры",
    "Извинись передо мной за свои плохие слова",
    "Иногда я смотрю на своих друзей и думаю, как было бы круто БЫТЬ БОГАТЫМ",
    "Исследование: коммунистами чаще всего становятся люди с низким IQ",
    "Как дела?",
    "Как же у тебя в голове насрано",
    "Как кыргызы относятся к акаеву??? Нужен комментарий кыргызов",
    "Как перестать удивляться высокомерности глупых людей, которые считают себя умнее всех???",
    "Как правильно ставить запятые???? Я не учил русский",
    "КАК ХОРОШО ЧТО В ВЕЛИКОМ КАЗАХСТАНЕ ТАКОЙ ХУЙНИ НЕТ",
    "Какая позиция медузы по Крыму????",
    "Когда в Беларуси протесты?",
    "Когда русские перестанут терпеть?",
    "Когда украинцы вернут Крым????",
    "Кринж",
    "Кто изобрел нарды???",
    "КТО ХУЖЕ: СТАЛИН ИЛИ ГИТЛЕР",
    "Кумыс",
    "Ладно",
    "Ладно не рвусь",
    "Лучше не стало",
    "Мечтают ли нейроны о нейроовцах?",
    "Может хватит рваться????",
    "Можешь ЗАВАЛИТЬ ебало?",
    "Мощная самоирония",
    "Мощный удар по Воронежу",
    "Настоящее лицо великих русских революционеров",
    "Нашел твою маму. Что с ней делать???",
    "Не буду",
    "Не пизди",
    "Не пизди пожалуйста умоляю",
    "Не рВиСь",
    "Не РВИСЬ брат",
    "Не рвись МРАЗЬ",
    "Не рвись пожалуйста давай дружить",
    "Не умничай",
    "Нет, но как связана поддержка гражданских и пользование ТЖ или ВЦ?",
    "Нет. В чем я не прав?",
    "Нихуя ты умный братан",
    "Нож в спину",
    "НУ И В ЧЕМ ОН НЕ ПРАВ???",
    "НУ И В ЧЕМ ТЫ НЕ ПРАВ?",
    "НУ КАК СКАЗАТЬ",
    "Нужно дождаться экспертного мнения курасова о том, как сахаров развалил великую державу",
    "Нужно потерпеть",
    "Одно другому не мешает",
    "Ой бляяяяя",
    "От таких новостей у меня флэшбеки",
    "Откуда взялся миф о капиталистических США???",
    "Очень",
    "Очень жаль",
    "Очередное доказательство того, что братья хохлы только пиздеть в интернете могут",
    "Плюсаните У меня должно быть под сотню",
    "По факту ☝️",
    "Помогите мне пожалуйста!",
    "Попробуй не заходить на тж",
    "Порвался жаль",
    "Порвался, жаль",
    "Порвался?))))))",
    "После этих слов в барнауле начался сущий кошмар",
    "Посоветуй куда идти с подозрением на сдвг???",
    "Почему Дмитрий Орлов забанен несправедливо?",
    "Почему злые олигархи не дади сталину построить в СССР демократию??????",
    "Почему из-за путинской твари спутник не котируется за рубежом??????? Кто отвечает??????",
    "Почему Лукашенко терпит этих белорусов???",
    "Почему маленькие девочки пишут о себе в мужском лице??",
    "Почему мартингал перестал рваться? Уехал на своей лодке в закат?",
    "Почему мужчины ставят себе женщин на аватарки?",
    "Почему муслимы бухают, но свинину не едят??",
    "Почему на дтф сидят так много обоссанных коммуняк?",
    "Почему Навальный не записывает Навальный лайв из колонии???",
    "Почему представитель народа-жертвы гододмора защищает колониализм??",
    "Почему так мало Навального????",
    "Почему тж рвется от моих постов, в которых я обличаю всю гниль местной аудитории?",
    "Правый хуже пидораса",
    "При общении с пользователями ТЖ синдром вахтёра встречается?",
    "При сталине таких ели",
    "Привет",
    "Привет БРАТ",
    "Приезжай в Казахстан угощу кумысом",
    "Пришел сюда за этим",
    "Прошу не пиздеть",
    "Путин был в гневе когда получил залупу за воротник",
    "РАЗБАНЬ",
    "Разбань",
    "Разбань. ЗАЧЕМ ОБИЖАЕШЬСЯ???",
    "Разбаньте!!!",
    "Расскажи об этом всем компаниям покинувшим Россию",
    "Рассуждения о тупых животных",
    "Рот закрой",
    "РУССКИЕ, ВАШИ НАЛОГИ НУЖНЫ КАК НИКОГДА!!!!",
    "С чего вдруг я русофоб????",
    "Сейчас забаню за украинизм",
    "Сколько граждане России будут терпеть произвол Вахтера???",
    "Сколько граждане России будут терпеть произвол силовиков???",
    "Сколько часов в неделю работает вахтер?",
    "Слабак",
    "Слишком умно",
    "Согласен",
    "Спасибо всем за поддержку!!!",
    "Спасибо люблю целую",
    "Спроси у лося зачем он порвался и добавил меня в чс?",
    "Сразу видно, украинские корни",
    "СРОЧНО! ПРЯМАЯ ЛИНИЯ С ДМИТРИЕМ ОРЛОВЫМ. ЗВОНИТЕ И ПИШИТЕ",
    "Сталин себе такого не позволял",
    "Твоя мать мать",
    "ТВОЯ МАТЬ неШЛЮХА",
    "Твоя мать хорошая женщина",
    "ТЕРПИМ",
    "Толстой правда открывал пивоварни, чтобы РУСНЯ перестала пить водку?",
    "Тошнит и кружится голова. Что делать?",
    "Травля",
    "Травля жаль",
    "Травля, жаль",
    "Ты бы прошел тест на руssкого?",
    "Ты был в Крыму??",
    "Ты есть в чате дыни?",
    "Ты зачем порвался?",
    "Ты не прав",
    "Ты прав",
    "Ты привился?",
    "Ты смотрел боку но пико?",
    "Ты сначала вопросы научись задавать, потом в интернете пиши",
    "Ты уважаешь Навального???",
    "Ты узбек? Давай дружить. Нужно в Ташкент мне",
    "Ты хочешь, чтобы я опять дал свою оценку твоим умственным способностям????????",
    "Умный дохуя?",
    "Унижу тебя на глазах у всего тж. Ты этого хочешь????",
    "Хватит писать такие комментарии. Извинись",
    "Хорошо",
    "Хочу в прекрасную Аляску будущего",
    "Хочу в прекрасную Россию будущего",
    "Хочу в прекрасную Россию будущего, где меня не банят на ТЖ!!!",
    "Хочу плакать. Что делать??",
    "Хочу чтобы Казахстан стал 51 штатом США",
    "Че рвешься?",
    "Че ты рвешься????? Успокойся",
    "Че хотел?",
    "Чей твинк и почему он не в бане?",
    "Чей?",
    "Чет ты меня заебал. Ты по делу что-то скажешь?",
    "Чечня превыше всего",
    "Что всё это означает?",
    "Что здесь написано?? Что означают эти цифры????",
    "Что здесь написано???",
    "Что изменилось на тж за эту неделю?¿??",
    "Что не так с клоунами на дтф?",
    "Что не так с русскими???",
    "Что не так с этим постом?",
    "Что произошло бы с Россией, если бы РУССКИЕ перестали терпеть????",
    "Что произошло бы с ТЖ, если бы пользователи перестали терпеть мой бан????",
    "Это была провокация, молодец",
    "Это наброс. Не бери в голову.",
    "Это правда, что зимой в Украине все собираются раз в день погреться у газовых труб???",
    "Это точно кремлеботы сидят и клепают",
    "Это человек из моего чс. Они не могут не рваться",
    "Это я тебя порвал??",
    "Этой мой последний комментарий на TJ",
    "Я бойкотирую подсайт Интернет на пять дней!!!!",
    "Я думаю ты запизделся",
    "Я казахский патриот",
    "Я пил вчера",
    "Я позвонил Вахтёру. Он меня забанил",
    "Я позвонил Вахтёру. Он порвался",
    "Я ПОПУТАЛ БЕРЕГА УЖЕ ИЛИ НЕТ????",
    "Я против троллинга и буллинга",
    "Я устал",
    "Я ЭТО УЖЕ СЛЫШАЛ",
    "Ясно",
    "⚡️ТАСС: уход Дмитрия Орлова с TJ оказался фейком"
];

const UNCOMMON_TEXT: &'static [&'static str] = &[
    "JAL",
    "В чем я не прав??",
    "Говоришь как Навальный",
    "Двойные стандарты, жаль",
    "Жалко ущемленных славян",
    "Жаль",
    "Жаль бедной Украине это не помогает",
    "Зачем порвался?",
    "Кринж",
    "Ладно",
    "Ладно не рвусь",
    "Не пизди",
    "Не РВИСЬ брат",
    "Не рвись МРАЗЬ",
    "Нихуя ты умный братан",
    "Очень",
    "Очень жаль",
    "По факту ☝️",
    "Порвался жаль",
    "Порвался, жаль",
    "Порвался?))))))",
    "Травля жаль",
    "Травля, жаль",
    "Ты зачем порвался?",
    "Чет ты меня заебал. Ты по делу что-то скажешь?",
];

const TRIGGERS: &'static [&'static str] = &["нейроорлов", "@Нейроорлов", "@neyroorlov_bot", "нейро орлов"];
const TRIGGERS_JAL: &'static [&'static str] = &["жаль", "жалко", "рвись", "порвался"];

use std::env;

use rand::Rng;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::UserId;
use serenity::prelude::*;
use parking_lot::RwLock;

struct Porvaha(RwLock<Option<UserId>>);

fn random_idx<T>(v: &[T]) -> &T {
    let idx = rand::thread_rng().gen_range(0..v.len());
    &v[idx]
}

#[async_trait]
impl EventHandler for Porvaha {
    async fn message(&self, ctx: Context, msg: Message) {
        
        let id = *self.0.read();

        if msg.author.id == id.unwrap() {
            return;
        }
        
        if msg.mentions_me(&ctx.http).await.unwrap_or(false) {
            let reply_to = if let Some(ref rmsg) = msg.referenced_message {
                if msg.content.starts_with("@Нейроорлов") && msg.content.ends_with("@Нейроорлов") {
                    rmsg
                } else {
                    &msg
                }
            } else {
                &msg
            };
            
            if let Err(why) = reply_to.channel_id.send_message(&ctx.http, |m| {
                m.reference_message(reply_to)
                    .content(random_idx(&MAIN_PHARSES))
            }).await {
                println!("Failed to send message: {:?}", why);
            }
            return;
        }

        let lowercase = msg.content.to_lowercase();
        if TRIGGERS.iter().any(|t| lowercase.contains(t)) {
            if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
                m.reference_message(&msg)
                    .content(random_idx(&COMMON_TEXT))
            }).await {
                println!("Failed to send message: {:?}", why);
            }
            return;
        }

        if lowercase.split(&[' ', ',']).next().iter().any(|m| TRIGGERS_JAL.contains(m)) 
        {
            if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
                m.reference_message(&msg)
                    .content(random_idx(&UNCOMMON_TEXT))
            }).await {
                println!("Failed to send message: {:?}", why);
            }
            return;
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        let mut guard = self.0.write();
        *guard = Some(ready.user.id);
        drop(guard);
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client =
        Client::builder(&token, intents).event_handler(Porvaha(RwLock::new(None))).await.expect("Err creating client");
    
    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
