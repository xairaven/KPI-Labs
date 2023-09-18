import sys

from telethon import TelegramClient, errors
from telethon.tl.functions.channels import InviteToChannelRequest

import json_loader
import telegram
import xlsx

settings = json_loader.get_data("settings.json")

account_data = settings["account"]
chat_data = settings["chat"]
table_settings = settings["table-settings"]
rows_dict = settings["rows"]

client = TelegramClient('account', api_id=account_data["api_id"], api_hash=account_data["api_hash"])


async def main():
    if len(sys.argv) != 2:
        raise ValueError("There must be 1 argument: path to .xlsx file")

    excel_file_name = sys.argv[1]

    adding_contacts_allowed = input("Can a script add people to contacts (only affects if there is a column with "
                                    "tags)?\n"
                                    "They will be deleted from contacts after adding to chat.\n"
                                    "This may lead to account ban.\n"
                                    "(Y/n): ") in {"Y", "y"}

    # Opening workbook, choosing active sheet, taking specified row
    persons = xlsx.get_table(excel_file_name, rows_dict, start_from=table_settings["start-reading-from-row"])

    # Get channel/mega-chat entity for inviting to it
    channel = await client.get_input_entity(int(chat_data["id"]))
    # User function for putting user entities in person's ["entity"].
    await telegram.get_users(client, persons, using_tags="tag" in list(rows_dict.keys()))

    # Get user entities that are not in contacts for using technique "Add and then delete from contacts"
    not_in_contacts = list(filter(lambda x: x["entity"].contact is False, persons))
    if adding_contacts_allowed:
        await telegram.add_to_contacts(client, not_in_contacts)
    else:
        for person in not_in_contacts:
            print("User {} cannot be added to chat without permission to adding to contacts.".format(person["surname"]))
            persons.remove(person)

    # If you call a method with a list, responses from the API will be lost
    for person in persons:
        try:
            await client(InviteToChannelRequest(channel, [person["entity"]]))
        except errors.UserPrivacyRestrictedError as e:
            # Privacy -> Groups & Channels -> Nobody
            print("({}) User's privacy settings do not allow you to invite him/her to chat".format(person["phone"]))
        except errors.UserNotMutualContactError as e:
            # User was already deleted from the chat. You need to be mutual contact
            print("({}) The provided user is not a mutual contact. Cannot add to chat.".format(person["phone"]))
        except Exception as e:
            print(e)

    # Entities already added to contacts if `adding_contacts_allowed`. So, they should be deleted
    if adding_contacts_allowed:
        await telegram.delete_from_contacts(client, not_in_contacts)
        for person in not_in_contacts:
            print("User {} was added and deleted from contacts".format(person["surname"]))

    print("Done!")


with client:
    client.loop.run_until_complete(main())
