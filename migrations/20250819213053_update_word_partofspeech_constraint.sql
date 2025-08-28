ALTER TABLE "vocabulary" 
    DROP CONSTRAINT "vocabulary_part_of_speech_check",
    ADD CONSTRAINT "vocabulary_part_of_speech_check" CHECK (part_of_speech IN ('noun', 'verb', 'adjective', 'adverb', 'pronoun', 'preposition', 'conjunction', 'interjection'));
