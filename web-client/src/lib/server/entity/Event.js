export default function (sequelize, DataTypes) {
	return sequelize.define(
		'Event',
		{
			id: { type: DataTypes.BIGINT, primaryKey: true },
			name: { type: DataTypes.TEXT, allowNull: false },
			guildId: { type: DataTypes.BIGINT, allowNull: false },
			ownerId: { type: DataTypes.BIGINT, allowNull: true }
		},
		{
			sequelize,
			tableName: 'event',
			timestamps: false
		}
	);
}
