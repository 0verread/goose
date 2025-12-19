package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use:   "pancake",
	Short: "todo for building bigger feature",
}

var versionCmd = &cobra.Command{
	Use:   "version",
	Short: "check pancake version",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Print(rootCmd.Use + " ")
	},
}

func Execute() {
	cobra.CheckErr(rootCmd.Execute())
}

func init() {
	rootCmd.AddCommand(versionCmd)
	rootCmd.AddCommand(initCmd)
}
