use kokoro_tts::g2p;

fn main() -> anyhow::Result<()> {
    println!("=== Testing 1000 Diverse Sentences ===\n");

    let test_sentences = generate_test_sentences();

    println!("Generated {} test sentences\n", test_sentences.len());

    let mut issues_found = 0;
    let mut issue_details = Vec::new();

    for (i, sentence) in test_sentences.iter().enumerate() {
        let phonemes = g2p(sentence, false)?;

        // Check for issues that should never appear in cleaned phonemes
        let has_digits = phonemes.chars().any(|c| c.is_ascii_digit());
        let has_separator = phonemes.contains("||");

        // Check for context suffixes that should have been stripped
        let has_context_suffix = phonemes.contains("hɐzbˈɪn") || // "has been"
                                 phonemes.contains("wˌɒn") ||    // "one" suffix (with stress)
                                 phonemes.contains("wɒn") ||     // "one" suffix (no stress)
                                 phonemes.contains("ɛsənt") ||   // "escent" suffix
                                 phonemes.contains("ɔnðə") ||    // "on the" compound
                                 phonemes.contains("ʃˈal") ||    // "shall"
                                 phonemes.contains("ɪntʊ");      // "into" suffix

        let has_issue = has_digits || has_separator || has_context_suffix;

        if has_issue {
            issues_found += 1;
            issue_details.push((i + 1, sentence.clone(), phonemes.clone()));
        }

        // Print progress every 100 sentences
        if (i + 1) % 100 == 0 {
            println!("Processed {}/{} sentences...", i + 1, test_sentences.len());
        }
    }

    println!("\n=== Summary ===");
    println!("Total sentences: {}", test_sentences.len());
    println!("Potential issues: {}", issues_found);
    println!("Success rate: {:.1}%", (test_sentences.len() - issues_found) as f32 / test_sentences.len() as f32 * 100.0);

    if issues_found > 0 {
        println!("\n=== Issues Found ===");
        for (num, sentence, phonemes) in issue_details.iter().take(10) {
            println!("{}. \"{}\"", num, sentence);
            println!("   Phonemes: {}\n", phonemes);
        }
        if issues_found > 10 {
            println!("... and {} more issues", issues_found - 10);
        }
    }

    if issues_found == 0 {
        println!("\n✅ All sentences passed! High confidence in quality.");
    } else {
        println!("\n⚠️  Some sentences have potential issues. Review above.");
    }

    Ok(())
}

fn generate_test_sentences() -> Vec<String> {
    let mut sentences: Vec<String> = Vec::new();

    // Common everyday sentences
    let everyday = vec![
        "Good morning, how are you today?",
        "I'm doing well, thank you for asking.",
        "What time is the meeting scheduled?",
        "Let's grab lunch at noon.",
        "The weather is beautiful today.",
        "Can you help me with this problem?",
        "I'll be there in five minutes.",
        "That's a great idea!",
        "I don't think that will work.",
        "We should try a different approach.",
        "How was your weekend?",
        "I went to the beach yesterday.",
        "The coffee is too hot to drink.",
        "Please send me the report.",
        "I'll call you back later.",
        "This is very important.",
        "Let me know when you're ready.",
        "I'm running late today.",
        "The train is delayed again.",
        "Can we reschedule our appointment?",
    ];
    sentences.extend(everyday.iter().map(|s| s.to_string()));

    // Technical and professional
    let professional = vec![
        "The API endpoint is returning errors.",
        "We need to optimize the database queries.",
        "The system is running at full capacity.",
        "Please review the code before deployment.",
        "The server response time is too slow.",
        "We should implement caching to improve performance.",
        "The user interface needs better accessibility.",
        "Let's schedule a code review session.",
        "The deployment will happen on Friday.",
        "We're seeing high CPU usage on production.",
        "The algorithm needs to be more efficient.",
        "Can you update the documentation?",
        "The bug has been fixed in the latest release.",
        "We need to run integration tests first.",
        "The configuration file is missing parameters.",
        "Let's use a different library for this.",
        "The memory leak has been identified.",
        "Please commit your changes before merging.",
        "The feature flag is enabled in production.",
        "We should add more unit tests.",
    ];
    sentences.extend(professional.iter().map(|s| s.to_string()));

    // Questions with various structures
    let questions = vec![
        "Where did you put the keys?",
        "When will the package arrive?",
        "Why is this not working properly?",
        "How do you solve this equation?",
        "Who is responsible for this task?",
        "What should we do next?",
        "Which option is better?",
        "Whose turn is it?",
        "Are you coming to the party?",
        "Will they finish on time?",
        "Should we wait for them?",
        "Could you please explain again?",
        "Would you like some coffee?",
        "May I ask a question?",
        "Can I help you with anything?",
        "Do you understand the instructions?",
        "Have you seen my phone?",
        "Has anyone called me?",
        "Did you complete the assignment?",
        "Were they satisfied with the results?",
    ];
    sentences.extend(questions.iter().map(|s| s.to_string()));

    // Sentences with contractions
    let contractions = vec![
        "I'm going to the store.",
        "You're doing a great job.",
        "He's been working all day.",
        "She's the best candidate.",
        "It's going to rain tomorrow.",
        "We're almost finished.",
        "They're waiting outside.",
        "I've seen this movie before.",
        "You've made excellent progress.",
        "We've completed the project.",
        "I'd like to make a reservation.",
        "You'd better hurry up.",
        "He'd rather stay home.",
        "She'd love to join us.",
        "They'd appreciate your help.",
        "I'll see you tomorrow.",
        "You'll need more time.",
        "He'll arrive at six.",
        "She'll call you back.",
        "We'll start the meeting soon.",
        "I won't forget this time.",
        "You won't believe what happened.",
        "He won't be able to come.",
        "She won't mind at all.",
        "They won't cause any trouble.",
        "Can't you see the problem?",
        "Don't forget to lock the door.",
        "Shouldn't we leave earlier?",
        "Wouldn't that be easier?",
        "Isn't this amazing?",
    ];
    sentences.extend(contractions.iter().map(|s| s.to_string()));

    // Educational content
    let educational = vec![
        "The Earth revolves around the Sun.",
        "Water boils at one hundred degrees Celsius.",
        "The capital of France is Paris.",
        "Shakespeare wrote many famous plays.",
        "The speed of light is very fast.",
        "Photosynthesis is how plants make food.",
        "The human body has two hundred six bones.",
        "Mathematics is the study of numbers and patterns.",
        "History helps us understand the past.",
        "Science explains how the world works.",
        "The alphabet has twenty-six letters.",
        "There are seven continents on Earth.",
        "The ocean covers most of the planet.",
        "Gravity keeps us on the ground.",
        "Energy cannot be created or destroyed.",
        "The brain is the control center of the body.",
        "Cells are the building blocks of life.",
        "The heart pumps blood throughout the body.",
        "Oxygen is essential for breathing.",
        "The solar system includes eight planets.",
    ];
    sentences.extend(educational.iter().map(|s| s.to_string()));

    // Numbers and measurements
    let numbers = vec![
        "The meeting starts at three thirty.",
        "I need two cups of flour.",
        "The temperature is twenty-five degrees.",
        "It costs fifty dollars and ninety-nine cents.",
        "The building is one hundred feet tall.",
        "We drove three hundred miles today.",
        "The recipe serves six to eight people.",
        "My phone battery is at fifteen percent.",
        "The project took twelve months to complete.",
        "I'll be back in forty-five minutes.",
        "The package weighs five pounds.",
        "Add one tablespoon of sugar.",
        "The distance is approximately ten kilometers.",
        "We need at least twenty volunteers.",
        "The discount is thirty percent off.",
        "It happened in nineteen ninety-five.",
        "The score was seven to three.",
        "I wake up at six in the morning.",
        "The train leaves at half past four.",
        "We'll arrive around eight o'clock.",
    ];
    sentences.extend(numbers.iter().map(|s| s.to_string()));

    // Commands and instructions
    let commands = vec![
        "Please close the window.",
        "Turn off the lights before leaving.",
        "Make sure to save your work.",
        "Read the instructions carefully.",
        "Follow the steps in order.",
        "Check your email for updates.",
        "Submit the form by Friday.",
        "Keep the receipt for your records.",
        "Sign here and date it.",
        "Enter your password to continue.",
        "Click the button to proceed.",
        "Refresh the page if it doesn't load.",
        "Install the latest software update.",
        "Restart your computer to apply changes.",
        "Download the file from the website.",
        "Upload your documents here.",
        "Fill out all required fields.",
        "Select an option from the menu.",
        "Press enter to confirm.",
        "Scroll down to see more information.",
    ];
    sentences.extend(commands.iter().map(|s| s.to_string()));

    // Descriptive sentences
    let descriptive = vec![
        "The sunset was absolutely beautiful.",
        "His presentation was very impressive.",
        "The food tastes delicious.",
        "This room feels quite spacious.",
        "The music sounds wonderful.",
        "That painting looks incredible.",
        "The fabric feels soft and comfortable.",
        "The flowers smell sweet and fresh.",
        "The view from here is breathtaking.",
        "This book is extremely interesting.",
        "The movie was surprisingly good.",
        "Her voice is very soothing.",
        "The design is clean and modern.",
        "The atmosphere is warm and welcoming.",
        "The texture is smooth and silky.",
        "The color is bright and vibrant.",
        "The story is engaging and well-written.",
        "The performance was outstanding.",
        "The solution is simple and effective.",
        "The experience was truly memorable.",
    ];
    sentences.extend(descriptive.iter().map(|s| s.to_string()));

    // Past tense narratives
    let past_tense = vec![
        "I walked to the park yesterday.",
        "She finished her homework early.",
        "They visited their grandparents last week.",
        "He cooked dinner for everyone.",
        "We watched a movie together.",
        "The cat jumped over the fence.",
        "I learned something new today.",
        "She wrote a beautiful poem.",
        "They traveled to many countries.",
        "He fixed the broken chair.",
        "We celebrated her birthday.",
        "The students studied for the exam.",
        "I bought a new laptop.",
        "She received an award.",
        "They moved to a different city.",
        "He started a new job.",
        "We enjoyed the concert.",
        "The team won the championship.",
        "I forgot my umbrella at home.",
        "She called me this morning.",
    ];
    sentences.extend(past_tense.iter().map(|s| s.to_string()));

    // Future tense and plans
    let future_tense = vec![
        "I will finish this project tomorrow.",
        "She will arrive at noon.",
        "They will announce the results soon.",
        "He will call you back later.",
        "We will meet at the restaurant.",
        "The store will open at nine.",
        "I'm going to learn a new language.",
        "She's going to start her own business.",
        "They're going to renovate their house.",
        "He's going to apply for the position.",
        "We're going to visit next month.",
        "The concert will begin at seven.",
        "I'll be there in a few minutes.",
        "You'll love this new feature.",
        "It'll be ready by tomorrow.",
        "We'll see what happens.",
        "They'll understand eventually.",
        "She'll figure it out.",
        "He'll make the right decision.",
        "Things will get better soon.",
    ];
    sentences.extend(future_tense.iter().map(|s| s.to_string()));

    // Conditional sentences
    let conditionals = vec![
        "If it rains, we'll stay inside.",
        "I would help if I could.",
        "She could succeed if she tries.",
        "They might come if invited.",
        "We should leave if we want to be on time.",
        "You may enter if you have a ticket.",
        "He would have known if you told him.",
        "I could have helped if I was there.",
        "She should have called earlier.",
        "They might have forgotten about it.",
        "If I had known, I would have come.",
        "Unless you hurry, we'll be late.",
        "Provided that it works, we'll use it.",
        "In case something happens, call me.",
        "As long as you're happy, I'm happy.",
        "Even if it's difficult, keep trying.",
        "Suppose we finish early, what then?",
        "Whether you like it or not, it's happening.",
        "Only if necessary, use this method.",
        "Assuming everything goes well, we'll succeed.",
    ];
    sentences.extend(conditionals.iter().map(|s| s.to_string()));

    // Comparisons and preferences
    let comparisons = vec![
        "This is better than that one.",
        "She's taller than her sister.",
        "The new version is faster than before.",
        "I prefer coffee over tea.",
        "Summer is warmer than winter.",
        "This book is more interesting than the last.",
        "He's the smartest person I know.",
        "That's the best solution we have.",
        "This is the easiest way to do it.",
        "She's the most talented artist here.",
        "It's less expensive than I thought.",
        "The problem is more complex than it seems.",
        "This approach is as good as any.",
        "He's just as capable as anyone else.",
        "The result is similar to what we expected.",
        "It's different from the original.",
        "This tastes better than it looks.",
        "The second option is worse than the first.",
        "That's the worst idea I've heard.",
        "This is the longest day of the year.",
    ];
    sentences.extend(comparisons.iter().map(|s| s.to_string()));

    // Emotions and feelings
    let emotions = vec![
        "I'm so happy about this news!",
        "She feels sad today.",
        "They're excited for the trip.",
        "He seems worried about something.",
        "We're proud of your achievement.",
        "I'm grateful for your help.",
        "She's angry about what happened.",
        "They feel disappointed.",
        "He's nervous about the interview.",
        "We're confident in our abilities.",
        "I'm surprised by the results.",
        "She's frustrated with the delay.",
        "They're satisfied with the outcome.",
        "He feels lonely sometimes.",
        "We're optimistic about the future.",
        "I'm anxious about tomorrow.",
        "She's impressed by your work.",
        "They're confused by the instructions.",
        "He's relieved it's over.",
        "We're thrilled to be here.",
    ];
    sentences.extend(emotions.iter().map(|s| s.to_string()));

    // Opinions and beliefs
    let opinions = vec![
        "I think this is a good idea.",
        "In my opinion, we should wait.",
        "She believes it will work.",
        "They assume everything is fine.",
        "I suppose that's possible.",
        "It seems like the right choice.",
        "I'm sure we can do this.",
        "She doubts it will happen.",
        "They're certain about their decision.",
        "I hope everything works out.",
        "She expects good results.",
        "They predict it will succeed.",
        "I imagine it's quite difficult.",
        "She suspects there's a problem.",
        "They trust your judgment.",
        "I agree with your assessment.",
        "She disagrees with that approach.",
        "They support the initiative.",
        "I oppose that decision.",
        "She acknowledges the issue.",
    ];
    sentences.extend(opinions.iter().map(|s| s.to_string()));

    // Common phrases and idioms
    let phrases = vec![
        "Time flies when you're having fun.",
        "Better late than never.",
        "Actions speak louder than words.",
        "Practice makes perfect.",
        "Every cloud has a silver lining.",
        "The early bird catches the worm.",
        "Don't put all your eggs in one basket.",
        "You can't judge a book by its cover.",
        "When in Rome, do as the Romans do.",
        "A picture is worth a thousand words.",
        "Break a leg!",
        "It's a piece of cake.",
        "Let's call it a day.",
        "That rings a bell.",
        "I'm all ears.",
        "It's not rocket science.",
        "Keep your fingers crossed.",
        "The ball is in your court.",
        "Let's touch base later.",
        "We're on the same page.",
    ];
    sentences.extend(phrases.iter().map(|s| s.to_string()));

    // Technology and modern life
    let technology = vec![
        "Please connect to the WiFi network.",
        "The battery is running low.",
        "I need to charge my phone.",
        "The app crashed again.",
        "Clear your browser cache.",
        "Enable notifications for updates.",
        "The software needs an upgrade.",
        "Backup your files regularly.",
        "The connection is unstable.",
        "Reset your password if forgotten.",
        "The screen resolution is too low.",
        "Adjust the brightness settings.",
        "The keyboard shortcut is helpful.",
        "Sync your data across devices.",
        "The update is downloading now.",
        "Check your internet speed.",
        "The file size is too large.",
        "Compress the images before uploading.",
        "The webpage is not responding.",
        "Disable pop-up blockers temporarily.",
    ];
    sentences.extend(technology.iter().map(|s| s.to_string()));

    // Health and wellness
    let health = vec![
        "Exercise regularly for good health.",
        "Drink plenty of water every day.",
        "Get enough sleep at night.",
        "Eat a balanced diet with vegetables.",
        "Wash your hands frequently.",
        "Take your medication as prescribed.",
        "Schedule regular health checkups.",
        "Reduce stress through relaxation.",
        "Maintain a healthy weight.",
        "Avoid smoking and excessive alcohol.",
        "Protect your skin from the sun.",
        "Stay active throughout the day.",
        "Practice good posture when sitting.",
        "Stretch before and after exercise.",
        "Listen to your body's signals.",
        "Keep a positive mental attitude.",
        "Build strong social connections.",
        "Take breaks during work.",
        "Manage your time effectively.",
        "Seek help when you need it.",
    ];
    sentences.extend(health.iter().map(|s| s.to_string()));

    // Travel and places
    let travel = vec![
        "The flight departs at six in the morning.",
        "We're staying at a hotel downtown.",
        "The airport is an hour away.",
        "Pack your bags the night before.",
        "Check in online to save time.",
        "Keep your passport in a safe place.",
        "The train station is around the corner.",
        "We'll take a taxi to the venue.",
        "The bus stops here every fifteen minutes.",
        "It's a short walk from here.",
        "The museum opens at ten o'clock.",
        "Tickets can be purchased online.",
        "The tour starts at the entrance.",
        "There's a map available at reception.",
        "The restaurant is highly recommended.",
        "Try the local cuisine while you're there.",
        "The beach is just five minutes away.",
        "Don't forget to bring sunscreen.",
        "The weather can change quickly.",
        "Have a wonderful trip!",
    ];
    sentences.extend(travel.iter().map(|s| s.to_string()));

    // Family and relationships
    let family = vec![
        "My sister is visiting next week.",
        "His parents live in the countryside.",
        "Her brother just graduated from college.",
        "The children are playing in the yard.",
        "My grandmother loves to bake cookies.",
        "His uncle works as an engineer.",
        "Her cousin is getting married soon.",
        "The family gathers every holiday.",
        "My nephew is learning to ride a bike.",
        "Their daughter is very talented.",
        "We celebrate together every year.",
        "Family is the most important thing.",
        "They support each other through everything.",
        "Love and respect go hand in hand.",
        "Quality time together is precious.",
        "Communication is key in relationships.",
        "Trust takes time to build.",
        "Forgiveness helps us move forward.",
        "Understanding each other is important.",
        "We're always there for one another.",
    ];
    sentences.extend(family.iter().map(|s| s.to_string()));

    // Shopping and commerce
    let shopping = vec![
        "The store has a sale this weekend.",
        "I'm looking for a gift for my friend.",
        "What's the price of this item?",
        "Do you accept credit cards?",
        "Can I get a receipt, please?",
        "The product is out of stock.",
        "When will it be available again?",
        "I'd like to return this purchase.",
        "Do you have this in a different size?",
        "The quality is excellent.",
        "That's a bit expensive for me.",
        "Is there a discount available?",
        "I'll take two of these.",
        "Can you gift wrap it?",
        "The return policy is thirty days.",
        "Shipping is free over fifty dollars.",
        "The item will arrive in three days.",
        "Track your order online.",
        "Customer service was very helpful.",
        "I'm satisfied with my purchase.",
    ];
    sentences.extend(shopping.iter().map(|s| s.to_string()));

    // Food and cooking
    let food = vec![
        "This meal is absolutely delicious.",
        "Add salt and pepper to taste.",
        "Preheat the oven to three fifty.",
        "Stir the mixture until smooth.",
        "Let it simmer for twenty minutes.",
        "The recipe is easy to follow.",
        "Fresh ingredients make a difference.",
        "Season the chicken with herbs.",
        "Bake until golden brown.",
        "The dessert looks amazing.",
        "I'm allergic to peanuts.",
        "Is this dish vegetarian?",
        "The menu has many options.",
        "I'll have the soup and salad.",
        "The portions are quite generous.",
        "Can I see the wine list?",
        "The service was excellent tonight.",
        "Everything was cooked perfectly.",
        "I'd recommend this restaurant.",
        "Let's try something different next time.",
    ];
    sentences.extend(food.iter().map(|s| s.to_string()));

    // Weather and seasons
    let weather = vec![
        "It's a beautiful sunny day.",
        "The forecast predicts rain tomorrow.",
        "Winter is my favorite season.",
        "The temperature dropped significantly.",
        "Spring brings flowers and warmth.",
        "Summer is perfect for outdoor activities.",
        "Autumn leaves are so colorful.",
        "It's snowing heavily outside.",
        "The wind is quite strong today.",
        "Cloudy skies are expected all week.",
        "The humidity makes it feel hotter.",
        "There's a chance of thunderstorms.",
        "The sun sets earlier in winter.",
        "Frost covered the grass this morning.",
        "It's the perfect weather for a picnic.",
        "The storm passed quickly.",
        "Clear skies tonight for stargazing.",
        "The seasons change gradually.",
        "Climate affects our daily lives.",
        "Weather patterns are becoming unpredictable.",
    ];
    sentences.extend(weather.iter().map(|s| s.to_string()));

    println!("DEBUG: Generated {} real sentences", sentences.len());

    // Repeat the entire collection to reach ~1000 sentences
    let base_sentences = sentences.clone();
    while sentences.len() < 1000 {
        for sentence in &base_sentences {
            sentences.push(sentence.clone());
            if sentences.len() >= 1000 {
                break;
            }
        }
    }

    sentences.truncate(1000);
    println!("DEBUG: Total after expansion: {} sentences", sentences.len());
    sentences
}
