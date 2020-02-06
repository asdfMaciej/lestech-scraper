use select::document::Document;
use select::predicate::{Class, Name};
use std::error::Error;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let html = reqwest::get("http://www.szkolanalesnej.edu.pl/lestech/rejestracja.php")
		.await?
		.text()
		.await?;

	let s_slice: &str = &*html; // Document library wants &'static str, and I have std::string
	let document = Document::from(s_slice);

	let mut teams = Vec::new();

	for game in document.find(Class("main-web-registered-div-players")).take(1) {
		for team in game.find(Name("div")) {
			let mut teamname: String = "".to_string();
			for team_name in team.find(Class("team-registered-teamname")) { // .next() didnt work
				teamname = team_name.text();
				break;
			}

			for player in team.find(Class("team-registered-player")) {
				let player_text = player.text();
				let words: Vec<&str> = player_text.split(" ").collect();

				let nick_slice = &words[1..words.len() - 1]; // exclude first and last word
				let mut nick = nick_slice.to_vec();
				nick.retain(|&element| element != ""); // delete empty words

				let fullname: String = format!("{} {}", words[0], words[words.len() - 1]);
				let nickname: String = nick.join(" ");

				let data = vec![teamname.clone(), fullname, nickname];
				teams.push(data);
			}
		}
	}

	let mut html = "
	<html>
	<head>
	<meta charset=\"UTF-8\">
	<title>Leś-Tech</title>
	<link rel=\"stylesheet\" href=\"style.css\">
	</head>
	<body>
	".to_string(); 
	for players in &teams {
		let player_template = format!("
		<div class=\"player\">
			<div class=\"team\">{}</div>
			<div class=\"name\">{}</div>
			<div class=\"nick\">{nick}</div>
			<a class=\"opgg\" href=\"https://eune.op.gg/summoner/userName={nick}\">opgg</a>
			<a class=\"maestry\" href=\"https://championmasterylookup.derpthemeus.com/summoner?summoner={nick}&region=EUNE\">maestry</a>
		</div>", players[0], players[1], nick = players[2]);
		html.push_str(&player_template);
	}
	html.push_str("
	</body>
	</html>
	");
	println!("{}", html);
	Ok(())
}
