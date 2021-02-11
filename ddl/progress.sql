drop table progresses

CREATE TABLE `progresses` (
  `game_id` int NOT NULL,
  `progress` int NOT NULL,
  `piecies` binary(16) NOT NULL,
  `time` datetime NOT NULL,
  PRIMARY KEY (`game_id`,`progress`),
  CONSTRAINT `game_id` FOREIGN KEY (`game_id`) REFERENCES `game_infos` (`game_id`)
) 