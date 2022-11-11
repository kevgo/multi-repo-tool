Feature: list subfolders matching a condition

  Background:
    Given I am in the "simple" example folder
    And no mrt configuration

  Rule: it displays the folders where the condition returns 0

    Scenario: happy path
      When running "m list ls go.mod"
      Then it prints:
        """
        ...

        2/3 folders match:
        1. go
        2. go_node
        """
      And it returns "success"
      And I am now in the "simple" example folder
      And there is no saved state

  Rule: displays help if called without command

    @this
    Scenario: call without command
      When running "m list"
      Then it prints:
        """
        ERROR: missing condition

        The "list" command displays all active directories in which the given CLI command returns exit code 0.
        It is a "dry run" of the "only" command.

        You forgot to tell me the CLI command I should run in each directory. You do it like this:

          m list <command>

        As an example, to find all codebases that are not Node.js:

          m list test -f package.json
        """
      And it returns "failure"
