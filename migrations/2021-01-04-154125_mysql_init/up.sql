CREATE TABLE `progresses` (
  `game_id` int NOT NULL,
  `piecies` binary(128) NOT NULL,
  `time` datetime NOT NULL,
  PRIMARY KEY (`game_id`,`piecies`)
);


CREATE TABLE `game_infos` (
  `game_id` int NOT NULL,
  `state` int NOT NULL,
  PRIMARY KEY (`game_id`)
);