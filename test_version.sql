load 'build/debug/quacking_quack.duckdb_extension';

create table words (word text);

insert into words values 
('running'),
('runner'),
('ran'),
('runs'),
('easily'),
('easier'),
('happily'),
('happiest'),
('flying'),
('flies'),
('cried'),
('crying'),
('studies'),
('studying'),
('studied'),
('playing'),
('played'),
('plays'),
('jumps'),
('jumping')
;

select word, quacking_quack(word) from words;