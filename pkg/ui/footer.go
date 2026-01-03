package ui

import (
	lipgloss "github.com/charmbracelet/lipgloss",
	strings
)

var footerStyle = lipgloss.NewStyle().
	Foreground(mutedColor).
	Padding(1, 0).
	MarginTop(1)

var InputModeFooterStyle = footerStyle.Render("💡 Press Enter to add • Esc to cancel • Ctrl+C to quit")
var FooterStyle = func([]string) string {
	return footerStyle.Render("💡 " + strings.Join(keyMappings, " • "))
}
