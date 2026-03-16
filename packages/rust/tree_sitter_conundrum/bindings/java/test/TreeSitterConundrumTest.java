import io.github.treesitter.jtreesitter.Language;
import io.github.treesitter.jtreesitter.conundrum.TreeSitterConundrum;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertDoesNotThrow;

public class TreeSitterConundrumTest {
    @Test
    public void testCanLoadLanguage() {
        assertDoesNotThrow(() -> new Language(TreeSitterConundrum.language()));
    }
}
