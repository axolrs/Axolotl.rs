import io.github.treesitter.jtreesitter.Language;
import io.github.treesitter.jtreesitter.axolotlgrammer.TreeSitterAxolotlgrammer;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertDoesNotThrow;

public class TreeSitterAxolotlgrammerTest {
    @Test
    public void testCanLoadLanguage() {
        assertDoesNotThrow(() -> new Language(TreeSitterAxolotlgrammer.language()));
    }
}
