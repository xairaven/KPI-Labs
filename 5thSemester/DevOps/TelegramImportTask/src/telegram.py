from telethon.tl.functions.contacts import AddContactRequest, DeleteContactsRequest


# User function for putting user entities in persons ["entity"].
async def get_users(client, persons, using_tags=False):
    for person in persons[:]:
        try:
            user = await client.get_entity(person["phone"])
            person["entity"] = user
            continue
        except (ValueError, TypeError) as e:
            # ValueError - Entity is not found by phone. We'll try to find by tag
            # TypeError - Empty "phone" field.
            if not using_tags or person["tag"] is None:
                if person["phone"] is not None:
                    print("Cannot find any entity corresponding to {}.".format(person["phone"]))
                else:
                    print("There's a person without phone and tag. That's dumb, isn't it?")
                persons.remove(person)
                continue
        except Exception as e:
            print(e)
            continue

        try:
            user = await client.get_entity(person["tag"])
            person["entity"] = user
            continue
        except Exception as e:
            print(e)
            persons.remove(person)
            continue


async def add_to_contacts(client, persons):
    for person in persons:
        user = person["entity"]
        await client(AddContactRequest(
            id=user.id,
            first_name=user.id,
            last_name="",
            phone=person["phone"]
        ))


async def delete_from_contacts(client, persons):
    ids = list(map(lambda x: x["entity"].id, persons))
    await client(DeleteContactsRequest(ids))
