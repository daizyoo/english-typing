pub type Questions = [(&'static str, &'static str); 30];

pub const fn questions(level: usize) -> Questions {
    match level {
        1 => LEVEL1,
        2 => LEVEL2,
        3 => LEVEL3,
        4 => LEVEL4,
        5 => LEVEL5,
        6 => LEVEL6,
        7 => LEVEL7,
        8 => LEVEL8,
        9 => LEVEL9,
        10 => LEVEL10,
        _ => panic!("Invalid level"),
    }
}

const LEVEL1: Questions = [
    ("apple", "りんご"),
    ("cat", "猫"),
    ("dog", "犬"),
    ("red", "赤"),
    ("blue", "青"),
    ("green", "緑"),
    ("I have a dog.", "私は犬を飼っています。"),
    ("I am happy.", "私は幸せです。"),
    ("You are kind.", "あなたは親切です。"),
    ("She is tall.", "彼女は背が高いです。"),
    ("He is smart.", "彼は賢いです。"),
    ("It is cold.", "寒いです。"),
    ("We are here.", "私たちはここにいます。"),
    ("They are friends.", "彼らは友達です。"),
    ("This is good.", "これは良いです。"),
    ("That is bad.", "それは悪いです。"),
    ("The sun is hot.", "太陽は暑いです。"),
    ("The moon is bright.", "月は明るいです。"),
    ("I like apples.", "私はりんごが好きです。"),
    ("She likes cats.", "彼女は猫が好きです。"),
    ("He likes dogs.", "彼は犬が好きです。"),
    ("It is a bird.", "それは鳥です。"),
    ("This is a pen.", "これはペンです。"),
    ("That is a book.", "それは本です。"),
    ("I see a tree.", "私は木を見ます。"),
    ("The car is red.", "車は赤いです。"),
    ("The sky is blue.", "空は青いです。"),
    ("I can run.", "私は走ることができます。"),
    ("You can jump.", "あなたはジャンプできます。"),
    ("We can sing.", "私たちは歌うことができます。"),
];

const LEVEL2: Questions = [
    ("I go to school.", "私は学校に行きます。"),
    ("He plays soccer.", "彼はサッカーをします。"),
    ("She reads books.", "彼女は本を読みます。"),
    ("We eat lunch.", "私たちは昼食を食べます。"),
    ("They drink water.", "彼らは水を飲みます。"),
    ("This is my house.", "これは私の家です。"),
    ("That is your car.", "それはあなたの車です。"),
    ("The dog runs fast.", "犬は速く走ります。"),
    ("I write a letter.", "私は手紙を書きます。"),
    ("She draws a picture.", "彼女は絵を描きます。"),
    ("He swims in the pool.", "彼はプールで泳ぎます。"),
    ("We talk every day.", "私たちは毎日話します。"),
    ("I wake up early.", "私は早く起きます。"),
    ("The boy jumps high.", "その男の子は高くジャンプします。"),
    ("The girl sings well.", "その女の子は上手に歌います。"),
    ("I see a big tree.", "私は大きな木を見ます。"),
    ("She has a blue bag.", "彼女は青いカバンを持っています。"),
    ("He drives a car.", "彼は車を運転します。"),
    ("We walk to the park.", "私たちは公園に歩いて行きます。"),
    ("The cat sleeps all day.", "猫は一日中寝ています。"),
    ("The bird flies high.", "鳥は高く飛びます。"),
    ("I study English.", "私は英語を勉強します。"),
    ("She cooks dinner.", "彼女は夕食を作ります。"),
    ("He watches TV.", "彼はテレビを見ます。"),
    ("We play games.", "私たちはゲームをします。"),
    ("They visit the zoo.", "彼らは動物園を訪れます。"),
    ("I like chocolate.", "私はチョコレートが好きです。"),
    ("She eats apples.", "彼女はりんごを食べます。"),
    ("He runs every day.", "彼は毎日走ります。"),
    (
        "She likes to read books in the evening.",
        "彼女は夕方に本を読むのが好きです。",
    ),
];

const LEVEL3: Questions = [
    ("What is your name?", "あなたの名前は何ですか？"),
    ("My name is John.", "私の名前はジョンです。"),
    ("How are you?", "お元気ですか？"),
    ("I am fine, thank you.", "元気です、ありがとう。"),
    ("Where do you live?", "どこに住んでいますか？"),
    ("I live in Tokyo.", "私は東京に住んでいます。"),
    ("What time is it?", "今何時ですか？"),
    ("It is five o'clock.", "5時です。"),
    ("Do you like pizza?", "ピザは好きですか？"),
    ("Yes, I like pizza.", "はい、ピザが好きです。"),
    ("Can you help me?", "手伝ってくれますか？"),
    ("Sure, what do you need?", "もちろん、何が必要ですか？"),
    ("What do you do?", "あなたの仕事は何ですか？"),
    ("I am a student.", "私は学生です。"),
    ("Do you have a pet?", "ペットを飼っていますか？"),
    ("Yes, I have a cat.", "はい、猫を飼っています。"),
    ("How old are you?", "何歳ですか？"),
    ("I am twenty years old.", "私は20歳です。"),
    ("Where is the station?", "駅はどこですか？"),
    (
        "Go straight and turn left.",
        "まっすぐ行って左に曲がってください。",
    ),
    ("What is your favorite color?", "好きな色は何ですか？"),
    ("My favorite color is blue.", "好きな色は青です。"),
    ("Can you speak English?", "英語を話せますか？"),
    ("Yes, I can speak a little.", "はい、少し話せます。"),
    ("What is your hobby?", "趣味は何ですか？"),
    ("I like reading books.", "本を読むことが好きです。"),
    ("Do you want coffee?", "コーヒーが欲しいですか？"),
    ("No, thank you.", "いいえ、大丈夫です。"),
    ("I will see you tomorrow.", "明日会いましょう。"),
    (
        "The weather is getting colder, so we should bring our jackets.",
        "天気が寒くなってきているので、ジャケットを持って行くべきです。",
    ),
];

const LEVEL4: Questions = [
    (
        "I went to the park yesterday.",
        "私は昨日公園に行きました。",
    ),
    (
        "She is reading a book right now.",
        "彼女は今本を読んでいます。",
    ),
    (
        "We are planning a trip to Kyoto.",
        "私たちは京都への旅行を計画しています。",
    ),
    (
        "He has already finished his homework.",
        "彼はすでに宿題を終えました。",
    ),
    (
        "They were playing soccer when I arrived.",
        "私が到着したとき、彼らはサッカーをしていました。",
    ),
    (
        "I will call you when I get home.",
        "家に着いたら電話します。",
    ),
    (
        "If it rains, we will stay inside.",
        "もし雨が降ったら、私たちは中にいます。",
    ),
    (
        "She doesn't like spicy food.",
        "彼女は辛い食べ物が好きではありません。",
    ),
    (
        "I have never been to France.",
        "私はフランスに行ったことがありません。",
    ),
    (
        "He is taller than his brother.",
        "彼は弟よりも背が高いです。",
    ),
    (
        "The movie was very interesting.",
        "その映画はとても面白かったです。",
    ),
    (
        "Can you tell me the way to the station?",
        "駅への行き方を教えてくれますか？",
    ),
    (
        "I usually go to bed at 11 PM.",
        "私は通常、夜11時に寝ます。",
    ),
    (
        "She always carries an umbrella.",
        "彼女はいつも傘を持っています。",
    ),
    (
        "I don't understand this question.",
        "私はこの質問がわかりません。",
    ),
    (
        "He is good at playing the piano.",
        "彼はピアノを弾くのが得意です。",
    ),
    (
        "The train was delayed due to the rain.",
        "雨のため電車が遅れました。",
    ),
    (
        "We need to finish this project soon.",
        "私たちはこのプロジェクトをすぐに終わらせる必要があります。",
    ),
    (
        "I forgot to bring my wallet.",
        "私は財布を持ってくるのを忘れました。",
    ),
    (
        "She was surprised to see the gift.",
        "彼女はそのプレゼントを見て驚きました。",
    ),
    (
        "He decided to study abroad next year.",
        "彼は来年留学することを決めました。",
    ),
    (
        "The weather is getting colder every day.",
        "天気が日ごとに寒くなっています。",
    ),
    (
        "She needs to improve her English skills.",
        "彼女は英語のスキルを向上させる必要があります。",
    ),
    (
        "I saw a beautiful rainbow after the rain.",
        "雨の後、美しい虹を見ました。",
    ),
    (
        "The store is closed on Sundays.",
        "その店は日曜日に閉まっています。",
    ),
    (
        "This cake tastes really delicious.",
        "このケーキは本当に美味しいです。",
    ),
    (
        "Do you know how to solve this problem?",
        "この問題の解き方を知っていますか？",
    ),
    (
        "I was waiting for the bus for an hour.",
        "私は1時間バスを待っていました。",
    ),
    (
        "Let's meet at the coffee shop at noon.",
        "正午に喫茶店で会いましょう。",
    ),
    (
        "He was so tired that he couldn't focus on the meeting, even though it was important.",
        "彼はとても疲れていたので、その会議が重要であったにもかかわらず、集中できませんでした。",
    ),
];

const LEVEL5: Questions = [
    (
        "I am looking forward to meeting you.",
        "お会いするのを楽しみにしています。",
    ),
    (
        "The book that I borrowed from the library is interesting.",
        "図書館から借りた本は面白いです。",
    ),
    (
        "He apologized for being late to the meeting.",
        "彼は会議に遅れたことを謝罪しました。",
    ),
    (
        "She can't decide whether to stay or leave.",
        "彼女は留まるべきか去るべきか決められません。",
    ),
    (
        "If I had more time, I would help you.",
        "もし時間があれば、手伝います。",
    ),
    (
        "I don't know why she is angry with me.",
        "なぜ彼女が私に怒っているのかわかりません。",
    ),
    (
        "The restaurant where we had dinner was fantastic.",
        "私たちが夕食を食べたレストランは素晴らしかったです。",
    ),
    (
        "I was surprised to hear the news about his promotion.",
        "彼の昇進のニュースを聞いて驚きました。",
    ),
    (
        "Although it was raining, we decided to go hiking.",
        "雨が降っていたにもかかわらず、私たちはハイキングに行くことにしました。",
    ),
    (
        "She asked me if I could help her with her project.",
        "彼女はプロジェクトを手伝えるかどうか私に尋ねました。",
    ),
    (
        "The movie that we watched last night was very exciting.",
        "昨晩見た映画はとても面白かったです。",
    ),
    (
        "He spent the entire afternoon fixing his car.",
        "彼は車を修理するのに午後全部を費やしました。",
    ),
    (
        "I will join you as soon as I finish my work.",
        "仕事が終わり次第、合流します。",
    ),
    (
        "She studies hard so that she can pass the exam.",
        "彼女は試験に合格するために一生懸命勉強します。",
    ),
    (
        "This is the best cake I have ever tasted.",
        "これは私が今までに食べた中で最高のケーキです。",
    ),
    (
        "The teacher explained the problem in detail.",
        "先生はその問題を詳しく説明しました。",
    ),
    (
        "He hasn't decided what to do after graduation.",
        "彼は卒業後に何をするかまだ決めていません。",
    ),
    (
        "It seems that the weather will improve tomorrow.",
        "天気は明日良くなるようです。",
    ),
    (
        "I didn't realize how difficult the task would be.",
        "その仕事がどれほど難しいか気づきませんでした。",
    ),
    (
        "If you need any help, feel free to ask me.",
        "もし助けが必要なら、遠慮なく聞いてください。",
    ),
    (
        "The song that she sang at the concert was beautiful.",
        "彼女がコンサートで歌った歌は美しかったです。",
    ),
    (
        "He suggested that we go out for dinner tonight.",
        "彼は今晩外で夕食を取ろうと提案しました。",
    ),
    (
        "Despite his busy schedule, he managed to visit us.",
        "忙しいスケジュールにもかかわらず、彼は私たちを訪ねてくれました。",
    ),
    (
        "I was impressed by how well he spoke English.",
        "彼の英語の上手さに感動しました。",
    ),
    (
        "The book is full of useful information about history.",
        "その本は歴史に関する役立つ情報でいっぱいです。",
    ),
    (
        "She told me that she would call me later.",
        "彼女は後で電話すると言いました。",
    ),
    (
        "We enjoyed the trip, although it was a bit tiring.",
        "少し疲れましたが、旅行を楽しみました。",
    ),
    (
        "The cake was so delicious that I couldn't stop eating.",
        "そのケーキはとても美味しくて、食べるのをやめられませんでした。",
    ),
    (
        "He is not only a good singer but also a talented actor.",
        "彼は良い歌手であるだけでなく、才能のある俳優でもあります。",
    ),
    (
        "Despite her fear of heights, she decided to climb the mountain with her friends.",
        "彼女は高所恐怖症にもかかわらず、友達と一緒に山を登ることに決めました。",
    ),
];

const LEVEL6: Questions = [
    (
        "She asked me whether I had seen her keys.",
        "彼女は私に鍵を見たかどうか尋ねました。",
    ),
    (
        "If I had known about the event earlier, I would have attended it.",
        "そのイベントについてもっと早く知っていたら、参加していたでしょう。",
    ),
    (
        "The house, which was built 50 years ago, needs renovation.",
        "50年前に建てられたその家は改装が必要です。",
    ),
    (
        "Although he was tired, he continued to work on the project.",
        "彼は疲れていましたが、そのプロジェクトに取り組み続けました。",
    ),
    (
        "I will call you as soon as I arrive at the airport.",
        "空港に着き次第、電話します。",
    ),
    (
        "She had already left when I arrived at the station.",
        "私が駅に着いた時、彼女はすでに出発していました。",
    ),
    (
        "The hotel where we stayed was very comfortable.",
        "私たちが泊まったホテルはとても快適でした。",
    ),
    (
        "The man who fixed my car did an excellent job.",
        "私の車を修理した男性は素晴らしい仕事をしました。",
    ),
    (
        "I was surprised that he knew so much about Japanese culture.",
        "彼が日本文化についてそんなに詳しいことに驚きました。",
    ),
    (
        "The company is looking for someone who can speak multiple languages.",
        "その会社は複数の言語を話せる人を探しています。",
    ),
    (
        "If I were you, I would apologize to her immediately.",
        "もし私があなたなら、すぐに彼女に謝ります。",
    ),
    (
        "The movie was so boring that we decided to leave halfway through.",
        "その映画はとても退屈だったので、途中で帰ることにしました。",
    ),
    (
        "I don't remember where I put my phone.",
        "私は携帯電話をどこに置いたのか覚えていません。",
    ),
    (
        "She works as a teacher while studying at university.",
        "彼女は大学で学びながら教師として働いています。",
    ),
    (
        "He didn't tell me why he was late.",
        "彼はなぜ遅れたのか私に言いませんでした。",
    ),
    (
        "Despite the heavy rain, they continued playing soccer.",
        "大雨にもかかわらず、彼らはサッカーを続けました。",
    ),
    (
        "The project will be delayed unless we work faster.",
        "もっと速く作業しなければ、そのプロジェクトは遅れるでしょう。",
    ),
    (
        "He is proud of what he has achieved in his career.",
        "彼は自分のキャリアで達成したことを誇りに思っています。",
    ),
    (
        "She couldn't remember where she had seen him before.",
        "彼女は以前彼をどこで見たのか思い出せませんでした。",
    ),
    (
        "The food was so spicy that I couldn't eat it.",
        "その食べ物はとても辛くて食べられませんでした。",
    ),
    (
        "He has been working here since he graduated from university.",
        "彼は大学を卒業して以来、ここで働いています。",
    ),
    (
        "The train was delayed due to a technical problem.",
        "電車は技術的な問題で遅れました。",
    ),
    (
        "I forgot to send an email to the client yesterday.",
        "昨日、顧客にメールを送るのを忘れました。",
    ),
    (
        "She didn't know how to solve the problem.",
        "彼女はその問題の解き方を知りませんでした。",
    ),
    (
        "I wonder if he will come to the party tomorrow.",
        "彼が明日のパーティーに来るかどうか気になります。",
    ),
    (
        "He promised to finish the report by next week.",
        "彼は来週までにそのレポートを終わらせると約束しました。",
    ),
    (
        "She is the kind of person who always helps others.",
        "彼女はいつも他人を助ける人です。",
    ),
    (
        "The place where we had lunch was very quiet.",
        "私たちが昼食を取った場所はとても静かでした。",
    ),
    (
        "If you don't mind, could you explain it again?",
        "もしよろしければ、もう一度説明していただけますか？",
    ),
    ("The company’s decision to expand internationally, while risky, could lead to tremendous growth if executed properly.", "その会社の国際的に拡大する決定はリスクが伴いますが、適切に実行されれば、驚くべき成長をもたらすかもしれません。")
];

const LEVEL7: Questions = [
    (
        "Had I known about the traffic jam, I would have taken a different route.",
        "渋滞について知っていれば、別の道を通ったでしょう。",
    ),
    (
        "She was so tired that she fell asleep as soon as she got home.",
        "彼女はとても疲れていたので、家に着くとすぐに眠りに落ちました。",
    ),
    (
        "The book, which I had been looking for, was finally available at the library.",
        "ずっと探していた本が、ようやく図書館で借りられました。",
    ),
    (
        "He suggested that we should meet earlier to prepare for the meeting.",
        "彼は会議の準備のために早めに会うべきだと提案しました。",
    ),
    (
        "Unless we leave now, we might miss the last train.",
        "今出発しないと、終電に乗り遅れるかもしれません。",
    ),
    (
        "The fact that he didn’t show up on time disappointed everyone.",
        "彼が時間通りに来なかったという事実がみんなを失望させました。",
    ),
    (
        "If I hadn't overslept, I would have caught the bus.",
        "寝坊しなければ、バスに間に合ったでしょう。",
    ),
    (
        "Despite being very talented, he is quite modest about his achievements.",
        "非常に才能があるにもかかわらず、彼は自分の業績についてかなり控えめです。",
    ),
    (
        "The reason why she refused the offer remains a mystery to us.",
        "彼女がその提案を断った理由は私たちには謎のままです。",
    ),
    (
        "The project, which had been delayed several times, was finally completed.",
        "何度も遅れていたそのプロジェクトがついに完成しました。",
    ),
    (
        "I will never forget the day when we first met.",
        "私たちが初めて会った日を決して忘れません。",
    ),
    (
        "He is saving money so that he can buy a new car next year.",
        "彼は来年新しい車を買うためにお金を貯めています。",
    ),
    (
        "Even though the weather was terrible, we managed to have a good time.",
        "天気がひどかったにもかかわらず、私たちは楽しい時間を過ごすことができました。",
    ),
    (
        "She was unaware that the event had been canceled until she arrived.",
        "彼女は到着するまでそのイベントがキャンセルされたことに気づきませんでした。",
    ),
    (
        "The children were excited because they had never seen snow before.",
        "子供たちは雪を見たことがなかったので興奮していました。",
    ),
    (
        "By the time we arrived at the station, the train had already left.",
        "私たちが駅に着いた時には、電車はすでに出発していました。",
    ),
    (
        "He apologized for forgetting to inform us about the change in schedule.",
        "彼はスケジュールの変更について知らせるのを忘れたことを謝罪しました。",
    ),
    (
        "If it hadn't been for her support, I couldn't have completed the task.",
        "彼女の支えがなかったら、その仕事を完了することはできなかったでしょう。",
    ),
    (
        "The building, which was constructed over 100 years ago, is now a museum.",
        "100年以上前に建設されたその建物は、今では博物館になっています。",
    ),
    (
        "I don’t know how he managed to solve such a complex problem.",
        "彼がどうやってそんなに複雑な問題を解決したのかわかりません。",
    ),
    (
        "Although the task was challenging, she completed it successfully.",
        "その仕事は難しかったけれど、彼女はそれを成功裏に終えました。",
    ),
    (
        "I wish I had studied harder when I was in school.",
        "学生の時にもっと一生懸命勉強していればよかったです。",
    ),
    (
        "The reason why he left the company was never revealed.",
        "彼がその会社を辞めた理由は決して明らかにされませんでした。",
    ),
    (
        "Since the road was closed, we had to find an alternative route.",
        "その道路が閉鎖されていたので、別のルートを探さなければなりませんでした。",
    ),
    (
        "The moment she heard the news, she burst into tears.",
        "彼女はそのニュースを聞いた瞬間、泣き出しました。",
    ),
    (
        "If we had followed his advice, we might have avoided the problem.",
        "彼の助言に従っていれば、その問題を避けられたかもしれません。",
    ),
    (
        "The teacher asked us to rewrite the essay to make it more concise.",
        "先生はエッセイをもっと簡潔にするように書き直すよう頼みました。",
    ),
    (
        "He explained the process in such a way that everyone could understand.",
        "彼は誰もが理解できるような形でそのプロセスを説明しました。",
    ),
    (
        "I had never experienced such hospitality until I visited their home.",
        "彼らの家を訪れるまで、そのようなもてなしを経験したことがありませんでした。",
    ),("Had we considered the long-term effects of our actions, we might have made different choices.", "もし私たちが行動の長期的な影響を考慮していたなら、異なる選択をしていたかもしれません。")
];

const LEVEL8: Questions = [
    ("Had it not been for his timely intervention, the situation could have escalated further.", "彼の迅速な介入がなかったら、状況はさらに悪化していたかもしれません。"),
    ("She speaks French fluently, which is impressive considering she only started learning it last year.", "彼女はフランス語を流暢に話しますが、昨年から学び始めたことを考えると驚きです。"),
    ("It is essential that the report be submitted before the deadline to avoid penalties.", "締切前に報告書を提出することがペナルティを避けるために不可欠です。"),
    ("No sooner had I entered the room than the phone started ringing.", "部屋に入るや否や、電話が鳴り始めました。"),
    ("Given the complexity of the issue, it might take weeks to resolve it.", "その問題の複雑さを考えると、解決するのに数週間かかるかもしれません。"),
    ("She insisted that the documents be reviewed thoroughly before making a decision.", "彼女は決定を下す前に書類を徹底的に確認するよう強く求めました。"),
    ("The conference, which was originally scheduled for next week, has been postponed indefinitely.", "来週に予定されていた会議は無期限に延期されました。"),
    ("Had we anticipated the challenges, we would have prepared differently.", "課題を予測していれば、別の準備をしていたでしょう。"),
    ("He acts as though he were the expert on the subject, despite his limited experience.", "彼は経験が乏しいにもかかわらず、その分野の専門家であるかのように振る舞います。"),
    ("If I were in your shoes, I would approach the matter more cautiously.", "もし私があなたの立場なら、その問題にもっと慎重に対処します。"),
    ("Her contributions, though often overlooked, have been vital to the team's success.", "彼女の貢献はしばしば見過ごされますが、チームの成功に不可欠でした。"),
    ("The project was completed on time, albeit with some minor issues.", "いくつかの小さな問題はありましたが、プロジェクトは時間通りに完了しました。"),
    ("The decision to close the factory was made after extensive consultation with stakeholders.", "工場を閉鎖するという決定は、利害関係者との十分な協議の後に行われました。"),
    ("Should you require further assistance, please do not hesitate to contact us.", "さらに支援が必要な場合は、遠慮なくご連絡ください。"),
    ("Although the task was daunting, she tackled it with remarkable determination.", "その仕事は大変でしたが、彼女は驚くべき決意でそれに取り組みました。"),
    ("Rarely have I encountered someone as dedicated to their work as she is.", "彼女ほど仕事に献身的な人に出会ったことはほとんどありません。"),
    ("He proposed several changes, all of which were rejected by the committee.", "彼はいくつかの変更を提案しましたが、それらはすべて委員会に却下されました。"),
    ("The event was so well-organized that it set a new standard for future occasions.", "そのイベントは非常によく組織されており、今後の基準を新たにしました。"),
    ("Unless we address the root cause of the issue, it is likely to recur.", "問題の根本原因に取り組まない限り、それが再発する可能性が高いです。"),
    ("She was reluctant to join the discussion, fearing that her ideas might be dismissed.", "彼女は自分の意見が却下されるかもしれないと恐れて、議論に参加することを渋っていました。"),
    ("The painting, believed to be a masterpiece, was discovered in an abandoned attic.", "傑作とされるその絵画は、放置された屋根裏部屋で発見されました。"),
    ("The speech was so compelling that the audience gave him a standing ovation.", "そのスピーチは非常に感動的で、聴衆は彼にスタンディングオベーションを送りました。"),
    ("It is imperative that we act swiftly to mitigate the potential damage.", "潜在的な被害を軽減するために迅速に行動することが必要不可欠です。"),
    ("Little did they know that their decision would have far-reaching consequences.", "彼らは自分たちの決定が広範な影響を及ぼすことをほとんど知らなかったのです。"),
    ("Not until I read the entire book did I understand its true meaning.", "その本をすべて読むまで、その真の意味を理解できませんでした。"),
    ("Despite his extensive experience, he remains open to learning new things.", "彼は豊富な経験を持ちながらも、新しいことを学ぶことに前向きです。"),
    ("Had she taken a different approach, the outcome might have been more favorable.", "彼女が別のアプローチを取っていれば、結果はもっと好ましいものになっていたかもしれません。"),
    ("The committee recommended that the proposal be revised before submission.", "委員会は、その提案を提出前に修正することを勧めました。"),
    ("While he is aware of the risks involved, he seems determined to proceed.", "彼は関わるリスクを認識しているものの、進める決意があるようです。"), 
    ("The research, which has been ongoing for several years, has produced some groundbreaking results that could change our understanding of human behavior.", "数年間続いているその研究は、人間の行動に対する理解を変える可能性がある画期的な結果を生み出しました。")
];

const LEVEL9: Questions = [
    ("Had the policy been implemented earlier, we could have avoided the current crisis.", "その政策がもっと早く実施されていれば、現在の危機を避けることができたかもしれません。"),
    ("The discovery of the new planet, which had been theorized for decades, confirmed many hypotheses about the universe.", "数十年にわたって仮定されていた新しい惑星の発見は、宇宙に関する多くの仮説を確認しました。"),
    ("If we had not taken that particular approach, the project would have failed completely.", "あの特定のアプローチを取らなかったなら、そのプロジェクトは完全に失敗していたでしょう。"),
    ("Only after extensive research was the team able to uncover the underlying cause of the problem.", "徹底的な調査を行った後で初めて、チームは問題の根本原因を明らかにすることができました。"),
    ("The decision to invest in renewable energy, though initially controversial, has proven to be beneficial in the long run.", "再生可能エネルギーへの投資という決定は、初めは論争を呼びましたが、長期的には有益であることが証明されました。"),
    ("Had he known the consequences, he would have acted very differently.", "もし彼が結果を知っていたら、まったく異なる行動を取っていたでしょう。"),
    ("If only we had considered all the possible outcomes, we could have made a better decision.", "もしすべての可能な結果を考慮していたなら、もっと良い決断ができたかもしれません。"),
    ("The legal implications of the case were not fully understood until the trial was underway.", "その事件の法的影響は、裁判が進行するまで完全には理解されませんでした。"),
    ("Had the government acted sooner, the environmental damage might have been minimized.", "政府がもっと早く行動していれば、環境へのダメージは最小限に抑えられたかもしれません。"),
    ("It is highly likely that the findings of the research will alter the way we approach the problem.", "その研究結果が問題へのアプローチを変える可能性が非常に高いです。"),
    ("Had the scientist's hypothesis been proven correct, the entire field of physics would have undergone a significant transformation.", "その科学者の仮説が正しいと証明されていれば、物理学の分野全体が大きく変革していたでしょう。"),
    ("The report, which was thoroughly reviewed by experts, revealed several unexpected findings.", "専門家によって徹底的にレビューされたその報告書は、いくつかの予期しない発見を明らかにしました。"),
    ("If it were not for the collaboration between these two organizations, the project would not have been successful.", "もしこれらの二つの組織間の協力がなかったら、そのプロジェクトは成功しなかったでしょう。"),
    ("The consequences of not adhering to these guidelines could be far-reaching, affecting both current and future generations.", "これらのガイドラインに従わないことの結果は広範囲に及び、現在および未来の世代にも影響を与える可能性があります。"),
    ("She would have never realized the severity of the issue if she had not taken a step back to reassess the situation.", "もし彼女が状況を再評価するために一歩引いていなければ、その問題の深刻さに気づかなかったでしょう。"),
    ("The company's decision to restructure, while difficult, was ultimately necessary for its survival.", "その会社の再編成という決定は、難しいものでしたが、最終的には生き残るために必要でした。"),
    ("Had they been more cautious, they might not have made such a costly mistake.", "もし彼らがもっと慎重だったなら、こんなに高い代償を払うような失敗はしなかったかもしれません。"),
    ("The documentary, which explored the effects of climate change, raised awareness among a global audience.", "気候変動の影響を探ったそのドキュメンタリーは、世界中の観客に意識を高めました。"),
    ("Had it not been for their collective efforts, the situation would have spiraled out of control.", "彼らの共同努力がなければ、状況は手に負えなくなっていたでしょう。"),
    ("The new policy, which aims to reduce carbon emissions, has been met with both support and opposition.", "二酸化炭素排出を削減することを目的とした新しい政策は、賛否両論を呼んでいます。"),
    ("If the team had been better equipped to handle the crisis, the damage could have been significantly reduced.", "もしそのチームが危機に対処するための準備ができていたなら、被害は大幅に減らせたでしょう。"),
    ("She was of the opinion that the benefits of the project far outweighed the risks involved.", "彼女はそのプロジェクトの利益が関わるリスクをはるかに上回ると考えていました。"),
    ("The lack of sufficient funding has led to delays in the completion of the project, despite our best efforts.", "十分な資金が不足しているため、最善を尽くしてもプロジェクトの完成が遅れています。"),
    ("Had the court ruled in their favor, the outcome of the trial would have been entirely different.", "もし裁判所が彼らに有利な判決を下していたなら、その裁判の結果はまったく異なっていたでしょう。"),
    ("The research, which took several years to complete, has provided valuable insights into the workings of the human brain.", "数年をかけて完成したその研究は、人間の脳の働きに関する貴重な洞察を提供しました。"),
    ("Should the negotiations fail, the company will have no choice but to pursue legal action.", "もし交渉が失敗すれば、会社は法的手段に訴えるしかないでしょう。"),
    ("Had the committee reviewed the data more thoroughly, the proposal would have been approved much sooner.", "委員会がデータをもっと徹底的にレビューしていたなら、その提案はもっと早く承認されていたでしょう。"),
    ("The researcher hypothesized that, if the experiment were conducted under different conditions, the results might vary significantly.", "その研究者は、実験が異なる条件下で行われたなら、結果が大きく異なるかもしれないと仮定しました。"),
    ("If the evidence presented had been stronger, the court’s verdict might have been in their favor.", "もし提示された証拠がもっと強力であったなら、裁判の判決は彼らに有利だったかもしれません。"),
    ("Had the committee made a more thorough evaluation of the data, the project’s outcome might have been more successful.", "もし委員会がデータをもっと徹底的に評価していたなら、そのプロジェクトの結果はもっと成功していたかもしれません。")
];

const LEVEL10: Questions = [
    ("Had the initial findings been more widely disseminated, the field might have advanced much faster.", "最初の調査結果がもっと広く伝えられていれば、その分野はもっと早く進展していたかもしれません。"),
    ("If the theory proves correct, it would represent a fundamental shift in our understanding of the universe.", "その理論が正しいと証明されれば、私たちの宇宙に対する理解に根本的な変革をもたらすことになります。"),
    ("The professor argued that, had the research been conducted under more rigorous conditions, the outcomes could have been significantly different.", "教授は、もしその研究がもっと厳密な条件下で行われていたなら、結果は大きく異なっていたかもしれないと主張しました。"),
    ("Were the findings to be corroborated by additional studies, they would likely alter the prevailing scientific paradigms.", "もしその結果が追加の研究によって裏付けられたなら、現在の科学的パラダイムを変える可能性があります。"),
    ("Had the historical context been more thoroughly examined, the interpretation of the events might have been more nuanced.", "歴史的背景がもっと徹底的に調査されていたなら、出来事の解釈はもっと繊細なものになっていたかもしれません。"),
    ("If the proposed legislation passes, it could lead to profound changes in the way healthcare is delivered.", "もし提案された法律が通過すれば、医療の提供方法に深い変化をもたらす可能性があります。"),
    ("Had the team applied a more interdisciplinary approach, they may have discovered solutions to the problem much sooner.", "もしチームがもっと学際的なアプローチを取っていれば、問題の解決策をもっと早く発見できたかもしれません。"),
    ("The findings, if proven accurate, will revolutionize current theories in the field of genetics.", "その結果が正確だと証明されれば、遺伝学の分野で現在の理論を革命的に変えることになります。"),
    ("If it were not for the contributions of early pioneers in the field, the modern understanding of neuroscience would be vastly different.", "その分野の初期の先駆者たちの貢献がなかったなら、現代の神経科学の理解は大きく異なっていたでしょう。"),
    ("The research methodology, if replicated in other contexts, has the potential to uncover new insights into the nature of consciousness.", "その研究方法論が他の文脈で再現されれば、意識の本質に関する新たな洞察を明らかにする可能性があります。"),
    ("Had the consequences of the decision been fully anticipated, the organization might have taken a different course of action.", "その決定の結果が完全に予測されていれば、その組織は異なる行動を取ったかもしれません。"),
    ("If the underlying assumptions of the model are incorrect, the results could be fundamentally flawed.", "もしそのモデルの根本的な仮定が誤っていれば、結果は根本的に欠陥があるかもしれません。"),
    ("The lack of consensus among the researchers suggests that further investigation is required before drawing any conclusions.", "研究者たちの間で合意が得られていないことは、結論を出す前にさらなる調査が必要であることを示唆しています。"),
    ("Had the data been analyzed with a more sophisticated algorithm, the insights gained could have been more comprehensive.", "もしそのデータがもっと洗練されたアルゴリズムで分析されていたなら、得られた洞察はもっと包括的だったかもしれません。"),
    ("The ethical implications of the research, though significant, have been largely overlooked by mainstream discourse.", "その研究の倫理的影響は重要であるにもかかわらず、主流の議論ではほとんど無視されています。"),
    ("If the theory were proven valid, it would radically alter our understanding of the relationship between mind and body.", "その理論が正当であると証明されれば、心と体の関係に対する私たちの理解は根本的に変わることになります。"),
    ("Had the researchers accounted for the cultural differences, their findings might have been interpreted in a more nuanced way.", "もし研究者たちが文化的な違いを考慮していたなら、その結果はもっと繊細な方法で解釈されたかもしれません。"),
    ("The historical significance of the artifact, had it not been lost, would have provided invaluable insight into ancient civilizations.", "その遺物が失われていなければ、古代文明に関する貴重な洞察を提供したであろうその歴史的意義は計り知れません。"),
    ("Had the team considered alternative methodologies, the project may have yielded more diverse and innovative outcomes.", "もしチームが代替の方法論を考慮していたなら、そのプロジェクトはもっと多様で革新的な成果を生み出したかもしれません。"),
    ("Were the regulations to be amended in the near future, it could significantly impact the global economy.", "もし規制が近い将来に改正されれば、それは世界経済に大きな影響を与える可能性があります。"),
    ("Had the researchers conducted a longitudinal study, they might have uncovered deeper patterns in human behavior over time.", "もし研究者たちが縦断的な研究を行っていたなら、時間の経過とともに人間の行動におけるより深いパターンを明らかにしていたかもしれません。"),
    ("If the proposed amendments to the constitution are ratified, it will redefine the political landscape of the nation.", "もし憲法改正案が承認されれば、それは国の政治的な風景を再定義することになります。"),
    ("Had the technology been available during the early stages of the project, the outcome would likely have been vastly different.", "もしその技術がプロジェクトの初期段階で利用可能であったなら、その結果は大きく異なっていたでしょう。"),
    ("If the hypothesis is proven to be correct, it would challenge many of the fundamental principles currently accepted in the field.", "その仮説が正しいと証明されれば、それはその分野で現在受け入れられている多くの基本的な原則に挑戦することになります。"),
    ("The ethical dilemma presented by the new technology, if left unaddressed, could have far-reaching consequences for society.", "新しい技術がもたらす倫理的ジレンマは、もし対処されなければ、社会に広範囲な影響を及ぼす可能性があります。"),
    ("Had the political leadership acted decisively at the onset of the crisis, the damage could have been mitigated considerably.", "もし政治的指導者が危機の初期に決断を下していたなら、被害はかなり軽減されていたかもしれません。"),
    ("The advancements in artificial intelligence, if properly regulated, could revolutionize industries ranging from healthcare to finance.", "人工知能の進歩は、適切に規制されれば、医療から金融に至るまでの産業を革命的に変える可能性があります。"),
    ("Were it not for the contributions of this particular research team, the field would have stagnated for decades.", "もしこの特定の研究チームの貢献がなかったなら、その分野は数十年間停滞していたでしょう。"),
    ("The interdisciplinary approach proposed by the panel, if implemented, could lead to breakthroughs in addressing global challenges.", "パネルが提案した学際的なアプローチは、実施されれば、世界的な課題に対処するための突破口を開くかもしれません。"),
    ("Had the risks been more clearly communicated, the public's reaction might have been more measured and less hostile.", "もしリスクがもっと明確に伝えられていれば、一般市民の反応はもっと冷静で、敵対的でなかったかもしれません。")
];
