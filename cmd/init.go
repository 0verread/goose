package cmd

import (
	"fmt"
	"os"

	"github.com/charmbracelet/bubbles/viewport"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
	"github.com/spf13/cobra"
)

var selectedItemStyle = lipgloss.NewStyle().Foreground(lipgloss.Color("170"))

type model struct {
	todo     []string
	cursor   int
	done     map[int]struct{}
	viewport viewport.Model
}

func initialModel() tea.Model {
	vp := viewport.New(80, 20)
	vp.SetContent("Hii from pancake")
	return model{
		todo:     []string{"Hii", "there", "mama"},
		viewport: vp,
	}
}

func (m model) Init() tea.Cmd {
	return nil
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {

	// Is it a key press?
	case tea.KeyMsg:

		// Cool, what was the actual key pressed?
		switch msg.String() {
		case "ctrl+c", "q":
			tea.ClearScreen()
			return m, tea.Quit
		case "up", "k":
			if m.cursor > 0 {
				m.cursor--
			}
		case "down", "j":
			if m.cursor < len(m.todo)-1 {
				m.cursor++
			}
		// add new item
		case "i":
			return m, tea.Quit

		case "enter", " ":
			_, ok := m.done[m.cursor]
			if ok {
				delete(m.done, m.cursor)
			} else {
				m.done[m.cursor] = struct{}{}
			}
		}
	}

	return m, nil
}

func (m model) View() string {
	// The header
	s := ""

	// Iterate over our choices
	for i, choice := range m.todo {

		// Is the cursor pointing at this choice?
		cursor := " " // no cursor
		if m.cursor == i {
			cursor = selectedItemStyle.Render(">")
			choice = selectedItemStyle.Render(choice)
		}

		// Is this choice selected?
		checked := " " // not selected
		if _, ok := m.done[i]; ok {
			checked = "√"
		}

		// Render the row
		s += fmt.Sprintf("%s [%s] %s\n", cursor, checked, choice)
	}
	m.viewport.SetContent(s)

	// Send the UI for rendering
	return m.viewport.View()
}

var initCmd = &cobra.Command{
	Use:   "init",
	Short: "start checklist for the current checkout branch",
	Run: func(cmd *cobra.Command, args []string) {
		p := tea.NewProgram(initialModel(), tea.WithAltScreen())
		if _, err := p.Run(); err != nil {
			fmt.Printf("Alas, there's been an error: %v", err)
			os.Exit(1)
		}
	},
}
