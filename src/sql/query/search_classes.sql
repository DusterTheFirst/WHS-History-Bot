SELECT
  `class`.`id`,
  `class`.`name`,
  `class`.`role`,
  `class`.`channel`,
  `group`.`id`,
  `group`.`name`,
  `group`.`channel_group`,
  `group`.`vc`
FROM `class`
LEFT OUTER JOIN `group` ON `class`.`group` = `group`.`id`
WHERE
  `class`.`name` LIKE ?1
  OR `group`.`name` LIKE ?1