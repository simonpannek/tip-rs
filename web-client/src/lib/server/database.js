import pg from 'pg';
import { Sequelize, DataTypes } from 'sequelize';

import { DB_USER, DB_PASSWORD, DB_HOSTNAME, DB_PORT, DB_NAME } from '$env/static/private';

import defineGuild from './entity/Guild.js';
import defineUser from './entity/User.js';
import defineEvent from './entity/Event.js';
import defineEventMember from './entity/EventMember.js';

// Connect to the database
const db_conn = new Sequelize(
	`postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOSTNAME}:${DB_PORT}/${DB_NAME}`,
	{
		dialectModule: pg,
		logging: false,
		pool: { max: 2, min: 0, idle: 0, acquire: 5000 }
	}
);

// Define all entities
const Guild = defineGuild(db_conn, DataTypes);
const User = defineUser(db_conn, DataTypes);
const Event = defineEvent(db_conn, DataTypes);
const EventMember = defineEventMember(db_conn, DataTypes);

// Define relationships
Guild.hasMany(Event, { foreignKey: 'guild_id' });
Event.belongsTo(Guild, { foreignKey: 'guild_id' });
User.hasMany(Event, { foreignKey: 'owner_id' });
Event.belongsTo(User, { as: 'owner', foreignKey: 'owner_id' });
User.belongsToMany(Event, { through: EventMember, foreignKey: 'user_id', otherKey: 'event_id' });
Event.belongsToMany(User, { through: EventMember, foreignKey: 'event_id', otherKey: 'user_id' });

export { db_conn, Guild, User, Event, EventMember };
